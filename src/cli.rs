use std::{collections::HashMap, path::Path};

use clap::Parser;
use clap_stdin::FileOrStdin;

type ValueMap = HashMap<String, serde_json::Value>;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(
        required_unless_present = "list_functions",
        help = "Path to the template file or - for stdin"
    )]
    pub template: Option<FileOrStdin>,

    #[arg(long, short, help = "Render a single flavor instead of all four")]
    pub flavor: Option<Flavor>,

    #[arg(long, help = "Set color overrides", value_parser = json_map::<ColorOverrides>)]
    pub color_overrides: Option<ColorOverrides>,

    #[arg(long, help = "Set frontmatter overrides", value_parser = json_map::<ValueMap>)]
    pub overrides: Option<ValueMap>,

    #[arg(long, help = "Dry run, don't write anything to disk")]
    pub dry_run: bool,

    #[arg(short, long, help = "List all Tera filters and functions")]
    pub list_functions: bool,

    #[arg(
        short,
        long,
        default_value = "json",
        help = "Output format of --list-functions"
    )]
    pub output_format: OutputFormat,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid JSON literal argument: {message}")]
    InvalidJsonLiteralArg { message: String },

    #[error("Invalid JSON file argument: {message}")]
    InvalidJsonFileArg { message: String },

    #[error("Failed to read file: {path}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl From<Flavor> for catppuccin::FlavorName {
    fn from(val: Flavor) -> Self {
        match val {
            Flavor::Latte => Self::Latte,
            Flavor::Frappe => Self::Frappe,
            Flavor::Macchiato => Self::Macchiato,
            Flavor::Mocha => Self::Mocha,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ColorOverrides {
    #[serde(default)]
    pub all: HashMap<String, String>,
    #[serde(default)]
    pub latte: HashMap<String, String>,
    #[serde(default)]
    pub frappe: HashMap<String, String>,
    #[serde(default)]
    pub macchiato: HashMap<String, String>,
    #[serde(default)]
    pub mocha: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Markdown,
    MarkdownTable,
}

fn json_map<T>(s: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    if Path::new(s).is_file() {
        let s = std::fs::read_to_string(s).map_err(|e| Error::ReadFile {
            path: s.to_string(),
            source: e,
        })?;
        serde_json::from_str(&s).map_err(|e| Error::InvalidJsonFileArg {
            message: e.to_string(),
        })
    } else {
        serde_json::from_str(s).map_err(|e| Error::InvalidJsonLiteralArg {
            message: e.to_string(),
        })
    }
}
