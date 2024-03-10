use std::collections::HashMap;

use catppuccin::FlavorName;
use clap::Parser as _;
use itertools::Itertools;
use whiskers2::{
    matrix::{self, Matrix},
    models, templating,
};

const FRONTMATTER_OPTS_SECTION: &str = "whiskers";

#[derive(Default, Debug, serde::Deserialize)]
struct RenderOpts {
    matrix: Option<Matrix>,
    filename: Option<String>,
}

impl RenderOpts {
    fn from_frontmatter(
        frontmatter: &HashMap<String, tera::Value>,
        only_flavor: Option<FlavorName>,
    ) -> anyhow::Result<Self> {
        #[derive(serde::Deserialize)]
        struct TemplateRenderOpts {
            matrix: Option<Vec<tera::Value>>,
            filename: Option<String>,
        }

        if let Some(opts) = frontmatter.get(FRONTMATTER_OPTS_SECTION) {
            let opts: TemplateRenderOpts = tera::from_value(opts.clone())?;
            let matrix = opts
                .matrix
                .map(|m| matrix::from_values(m, only_flavor))
                .transpose()?;
            Ok(Self {
                matrix,
                filename: opts.filename,
            })
        } else {
            Ok(Self::default())
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = whiskers2::cli::Args::parse();

    let Some(template_name) = args.template_path.file_name() else {
        return Err(anyhow::anyhow!("Template path must be a file"));
    };
    let template_name = template_name.to_string_lossy().into_owned();

    let doc = whiskers2::frontmatter::parse(&std::fs::read_to_string(&args.template_path)?)?;
    let render_opts = RenderOpts::from_frontmatter(&doc.frontmatter, args.flavor.map(Into::into))?;

    let palette = models::build_palette(args.hexcaps);
    let mut ctx = tera::Context::new();

    for (key, value) in &doc.frontmatter {
        ctx.insert(key, &value);
    }

    let mut tera = templating::make_engine();
    tera.add_raw_template(&template_name, &doc.body)?;

    // if a matrix is provided, we're doing a multi-file render
    if let Some(matrix) = render_opts.matrix {
        let Some(filename_template) = render_opts.filename else {
            anyhow::bail!("filename template is required for multi-file render");
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
            println!("would write {} bytes into {filename}", result.len());
        } else {
            std::fs::write(filename, result)?;
        }
    }

    Ok(())
}
