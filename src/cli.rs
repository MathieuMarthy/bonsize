use clap::Parser;
use std::path::PathBuf;

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
    pub max_depth: Option<u32>,

    // show the output as a sorted list instead of a tree
    #[arg(short = 's', long = "sorted", default_value_t = false)]
    pub show_as_sorted_list: bool,
}
