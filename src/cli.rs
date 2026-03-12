use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Debug, Clone)]
pub enum Sort {
    Asc,
    Desc,
}

// show the size of your directories and files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    // show only files
    #[arg(short = 'F', long = "file", default_value_t = false)]
    pub show_only_files: bool,

    // show only directories
    #[arg(short = 'D', long = "directory", default_value_t = false)]
    pub show_only_dir: bool,

    // max depth to display
    #[arg(short = 'd', long = "depth", default_value = None)]
    pub max_depth: Option<usize>,

    // show the output as a sorted list (asc or desc)
    #[arg(short = 's', long = "sorted", default_missing_value = "desc", num_args = 0..=1)]
    pub sort: Option<Sort>,

    // show the output in csv format
    #[arg(long = "csv", default_missing_value = ";", num_args = 0..=1)]
    pub csv: Option<char>,
    
    // use cache to speed up the scanning process (cache will be used if the same path is scanned again within a certain time frame)
    #[arg(short = 'c', long = "cache", default_value_t = false)]
    pub cache: bool,

    // hide error messages (e.g., permission denied)
    #[arg(short = 'q', long = "quiet", default_value_t = false)]
    pub quiet: bool,
}
