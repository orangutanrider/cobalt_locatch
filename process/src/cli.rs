use std::path::PathBuf;
use clap::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>, 

    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}