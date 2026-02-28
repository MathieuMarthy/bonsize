use clap::Parser;
use treesize_cli::cli::Cli;
use treesize_cli::display::{display, display_as_sorted_list};
use treesize_cli::scanner::tree_size::get_directory_size;

fn main() {
    let args = Cli::parse();

    println!("path: {:?}", &args.path);

    let dir = get_directory_size(&args.path).unwrap();

    if args.show_as_sorted_list {
        display_as_sorted_list(&dir, &args);
    } else {
        display(&dir, &args, 0);
    }
}
