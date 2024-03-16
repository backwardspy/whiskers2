use std::collections::HashMap;

use catppuccin::FlavorName;
use clap::Parser as _;
use itertools::Itertools;
use whiskers2::{
    context::merge_values,
    matrix::{self, Matrix},
    models, templating,
};

const FRONTMATTER_OPTIONS_SECTION: &str = "whiskers";

#[derive(Default, Debug, serde::Deserialize)]
struct TemplateOptions {
    version: Option<semver::VersionReq>,
    matrix: Option<Matrix>,
    filename: Option<String>,
}

impl TemplateOptions {
    fn from_frontmatter(
        frontmatter: &HashMap<String, tera::Value>,
        only_flavor: Option<FlavorName>,
    ) -> anyhow::Result<Self> {
        // a `TemplateOptions` object before matrix transformation
        #[derive(serde::Deserialize)]
        struct RawTemplateOptions {
            version: Option<semver::VersionReq>,
            matrix: Option<Vec<tera::Value>>,
            filename: Option<String>,
        }

        if let Some(opts) = frontmatter.get(FRONTMATTER_OPTIONS_SECTION) {
            let opts: RawTemplateOptions = tera::from_value(opts.clone())?;
            let matrix = opts
                .matrix
                .map(|m| matrix::from_values(m, only_flavor))
                .transpose()?;
            Ok(Self {
                version: opts.version,
                matrix,
                filename: opts.filename,
            })
        } else {
            Ok(Self::default())
        }
    }
}

fn main() -> anyhow::Result<()> {
    // parse command-line arguments & template frontmatter
    let args = whiskers2::cli::Args::parse();
    let doc = whiskers2::frontmatter::parse(&std::fs::read_to_string(&args.template_path)?)?;
    let template_opts =
        TemplateOptions::from_frontmatter(&doc.frontmatter, args.flavor.map(Into::into))?;

    // check that the template is compatible with this version of Whiskers
    let whiskers_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))?;
    if let Some(template_version) = template_opts.version {
        if !template_version.matches(&whiskers_version) {
            anyhow::bail!(
                "Template requires whiskers version {template_version}, but we're running {whiskers_version}",
            );
        }
    } else {
        eprintln!("Warning: No Whiskers version requirement specified in template.");
        eprintln!("This template may not be compatible with this version of Whiskers.");
        eprintln!();
        eprintln!("To fix this, add the minimum supported Whiskers version to the template frontmatter as follows:");
        eprintln!();
        eprintln!("---");
        eprintln!("whiskers:");
        eprintln!("    version: \"{whiskers_version}\"");
        eprintln!("---");
        eprintln!();
    }

    // merge frontmatter with command-line overrides and add to Tera context
    let mut frontmatter = doc.frontmatter;
    if let Some(overrides) = args.overrides {
        for (key, value) in overrides {
            frontmatter
                .entry(key)
                .and_modify(|v| {
                    *v = merge_values(v, &value);
                })
                .or_insert(tera::to_value(value)?);
        }
    }
    let mut ctx = tera::Context::new();
    for (key, value) in &frontmatter {
        ctx.insert(key, &value);
    }

    // build the Tera engine and palette
    let mut tera = templating::make_engine();
    let template_name = args.template_path.file_name().map_or_else(
        || "template".to_string(),
        |name| name.to_string_lossy().to_string(),
    );
    tera.add_raw_template(&template_name, &doc.body)?;
    let palette = models::build_palette(
        args.capitalize_hex,
        args.hex_prefix.as_deref(),
        args.color_overrides.as_ref(),
    )?;

    // if a matrix is provided, we're doing a multi-file render
    if let Some(matrix) = template_opts.matrix {
        let Some(filename_template) = template_opts.filename else {
            anyhow::bail!("Filename template is required for multi-file render");
        };
        render_multi_file(
            matrix,
            &filename_template,
            &ctx,
            &palette,
            &tera,
            &template_name,
            args.dry_run,
        )?;
    }
    // otherwise, we're doing a single-file render
    else {
        render_single_file(
            args.flavor.map(Into::into),
            &ctx,
            &palette,
            &tera,
            &template_name,
        )?;
    }

    Ok(())
}

fn render_single_file(
    flavor: Option<FlavorName>,
    ctx: &tera::Context,
    palette: &models::Palette,
    tera: &tera::Tera,
    template_name: &str,
) -> Result<(), anyhow::Error> {
    let mut ctx = ctx.clone();
    ctx.insert("flavors", &palette.flavors);
    if let Some(flavor) = flavor {
        let flavor = &palette.flavors[flavor.identifier()];
        ctx.insert("flavor", flavor);

        // also throw in the flavor's colors for convenience
        for (_, color) in flavor {
            ctx.insert(&color.identifier, &color);
        }
    }
    let result = tera.render(template_name, &ctx)?;
    print!("{result}");
    Ok(())
}

fn render_multi_file(
    matrix: HashMap<String, Vec<String>>,
    filename_template: &str,
    ctx: &tera::Context,
    palette: &models::Palette,
    tera: &tera::Tera,
    template_name: &str,
    dry_run: bool,
) -> Result<(), anyhow::Error> {
    let iterables = matrix
        .into_iter()
        .map(|(key, iterable)| iterable.into_iter().map(move |v| (key.clone(), v)))
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    for iterable in iterables {
        let mut ctx = ctx.clone();
        for (key, value) in iterable {
            // expand flavor automatically to prevent requiring:
            // `{% set flavor = flavors[flavor] %}`
            // at the top of every template.
            if key == "flavor" {
                let flavor: catppuccin::FlavorName = value.parse()?;
                let flavor = &palette.flavors[flavor.identifier()];
                ctx.insert("flavor", flavor);
            } else {
                ctx.insert(key, &value);
            }
        }
        let result = tera.render(template_name, &ctx)?;
        let filename = tera::Tera::one_off(filename_template, &ctx, false)?;

        if dry_run || cfg!(test) {
            println!(
                "Would write {} bytes into {filename}",
                result.as_bytes().len()
            );
        } else {
            std::fs::write(filename, result)?;
        }
    }

    Ok(())
}
