use std::{collections::HashMap, path::PathBuf};

use clap::Parser;

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

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
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

fn json_map<T>(s: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(s).map_err(Into::into)
}

type ValueMap = HashMap<String, serde_json::Value>;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub template_path: PathBuf,

    #[arg(long, short, help = "Render a single flavor instead of all four")]
    pub flavor: Option<Flavor>,

    #[arg(long, help = "Set color overrides", value_parser = json_map::<ColorOverrides>)]
    pub color_overrides: Option<ColorOverrides>,

    #[arg(long, help = "Set frontmatter overrides", value_parser = json_map::<ValueMap>)]
    pub overrides: Option<ValueMap>,

    #[arg(long, help = "Capitalize hex s")]
    pub hexcaps: bool,

    #[arg(long, help = "Dry run, don't write anything to disk")]
    pub dry_run: bool,
}
