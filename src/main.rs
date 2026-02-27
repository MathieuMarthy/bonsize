use clap::Parser;
use treesize_cli::cli::Cli;

fn main() {
    let args = Cli::parse();

    println!("path: {:?}", args.path);
}
