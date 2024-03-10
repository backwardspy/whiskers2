use std::path::PathBuf;

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

#[derive(Parser, Debug)]
pub struct Args {
    pub template_path: PathBuf,

    #[arg(
        value_enum,
        help = "Render a single flavor. Short for --overrides '{\"flavor\": [\"<flavor>\"]}'"
    )]
    pub flavor: Option<Flavor>,

    #[arg(long, help = "Capitalize hex strings")]
    pub hexcaps: bool,

    #[arg(long, help = "Dry run, don't write anything to disk")]
    pub dry_run: bool,
}
