use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{fs, path::Path, path::PathBuf};
use strum::Display;
use tabled::{
    Table, Tabled, settings::Color, settings::Style, settings::object::Columns,
    settings::object::Rows,
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

// Datatype to hold file metadata
#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size B")]
    len_bytes: u64,
    #[tabled(rename = "Last Modified")]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best ls command ever")]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));
    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if does_exist {
                if cli.json {
                    let get_files = get_files(&path);
                    println!(
                        "{}",
                        serde_json::to_string(&get_files)
                            .unwrap_or("cannot parse json".to_string())
                    )
                } else {
                    print_table(path);
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

fn get_files(path: &Path) -> Vec<FileEntry> {
    // Hold the return data
    let mut data = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }
    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e, %Y"))
            } else {
                String::default()
            },
        });
    }
}

fn print_table(path: PathBuf) {
    let get_files = get_files(&path);
    let mut table = Table::new(get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
}
