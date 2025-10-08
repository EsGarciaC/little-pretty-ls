use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size B")]
    len_bytes: u64,
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Pretty ls command")]
struct Args {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let args: Args = Args::parse();

    let path = args.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if args.json {
                let get_files = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&get_files).unwrap_or("Cannot parse json".to_string())
                );
            } else {
                print_table(path);
            }
        } else {
            println!("{}", "Path does not exist.".red());
        }
    } else {
        print!("{}", "Error reading directory".red());
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
    println!("{table}");
}

fn get_files(path: &PathBuf) -> Vec<FileEntry> {
    let mut data = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            map_data(&mut data, entry);
        }
    }
    data
}

fn map_data(data: &mut Vec<FileEntry>, entry: fs::DirEntry) {
    if let Ok(meta) = fs::metadata(entry.path()) {
        data.push(FileEntry {
            name: entry
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.size(),
            modified: if let Ok(modified) = meta.modified() {
                let date: DateTime<Utc> = modified.into();
                format!("{}", date.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}
