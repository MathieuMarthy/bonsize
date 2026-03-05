use clap::Parser;
use treesize_cli::cli::Cli;
use treesize_cli::display::output_format::OutputFormat;
use treesize_cli::scanner::get_directory_size;

fn main() {
    let args = Cli::parse();
    let dir = get_directory_size(&args.path);

    OutputFormat::from_args(&args.sort, args.csv).display(&dir, &args);
}
