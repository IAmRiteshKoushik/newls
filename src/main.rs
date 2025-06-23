use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}
