use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best ls command ever")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if does_exist {
                for file in get_files(&path) {
                    println!("{}", file);
                }
            }
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        // If it's existance can neither be confirmed nor denied because the
        // fs::exists() function errors out due to some reason
        println!("{}", "error reading directory".red());
    }
    // println!("{}", path.display());
}

fn get_files(path: &Path) -> Vec<String> {
    // Hold the return data
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                data.push(
                    file.file_name()
                        .into_string()
                        .unwrap_or("unknown name".into()),
                );
            }
        }
    }
    // return statement is not needed
    data
}
