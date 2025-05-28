use std::path::PathBuf;
use clap::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub list: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>, 

    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub filename_macro: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub ticket_macro: Option<PathBuf>,
}

// Fallback path
pub const CONFIG_FALLBACK: &str = "locatch_config.json";