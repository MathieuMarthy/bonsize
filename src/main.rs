use clap::Parser;
use bonsize::cli::Cli;
use bonsize::display::output_format::OutputFormat;
use bonsize::scanner::cache::{load_cache, save_cache};
use bonsize::scanner::file_model::FileModel;
use bonsize::scanner::{get_directory_size, ScanOptions};

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
