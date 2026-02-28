use clap::Parser;
use treesize_cli::cli::Cli;
use treesize_cli::display::display;
use treesize_cli::scanner::tree_size::get_directory_size;

fn main() {
    let args = Cli::parse();

    println!("path: {:?}", &args.path);

    let dir = get_directory_size(&args.path).unwrap();
    display(&dir, &args, 0);
}
