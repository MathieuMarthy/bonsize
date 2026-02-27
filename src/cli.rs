use clap::Parser;
use std::path::PathBuf;

// show the size of your directories and files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}
