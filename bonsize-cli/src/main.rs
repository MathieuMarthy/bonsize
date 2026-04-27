mod cli;
mod display;

use crate::cli::Cli;
use crate::display::output_format::OutputFormat;
use bonsize_core::scanner::cache::{load_cache, save_cache};
use bonsize_core::scanner::file_model::FileModel;
use bonsize_core::scanner::file_size::{ScanOptions, SizeType};
use bonsize_core::scanner::get_directory_size;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    let scan_options = ScanOptions {
        quiet: args.quiet,
        size_type: if args.use_logical_size { SizeType::Logical } else { SizeType::Physical },
    };

    let dir: FileModel;
    if args.cache
        && let Some(cached_dir) = load_cache(&args.path)
    {
        dir = cached_dir;
    } else {
        dir = get_directory_size(&args.path, &scan_options);

        if args.cache {
            save_cache(&args.path, &dir);
        }
    };

    OutputFormat::from_args(&args.sort, args.csv).display(&dir, &args);
}
