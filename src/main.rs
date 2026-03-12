use clap::Parser;
use treesize_cli::cli::Cli;
use treesize_cli::display::output_format::OutputFormat;
use treesize_cli::scanner::cache::{load_cache, save_cache};
use treesize_cli::scanner::file_model::FileModel;
use treesize_cli::scanner::{get_directory_size, ScanOptions};

fn main() {
    let args = Cli::parse();

    let scan_options = ScanOptions { quiet: args.quiet };

    let dir: FileModel;
    if args.cache && let Some(cached_dir) = load_cache(&args.path) {
        dir = cached_dir;
    } else {
        dir = get_directory_size(&args.path, &scan_options);

        if args.cache {
            save_cache(&args.path, &dir);
        }
    };

    OutputFormat::from_args(&args.sort, args.csv).display(&dir, &args);
}
