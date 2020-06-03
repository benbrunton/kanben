use std::env::var;
use dirs::home_dir;
use clap::{App, Clap};

use kv::{Store as KvStore, Config, Json};

mod commands;
mod opts;
mod store;
mod editor;
mod file;
mod board;

#[cfg(test)]
mod test;

use opts::Opts;
use store::PersistantStore;
use editor::FileEditor;
use file::FileReader;
use board::Board;

fn main() {
    let _app = App::new("kanben");

    let opts: Opts = Opts::parse();

    let home_path_bfr = home_dir().unwrap();
    let home_path = home_path_bfr.to_str().unwrap();
    let cfg_location = format!("{}{}", home_path, "/.kanben");
    let cfg = Config::new(&cfg_location);
    let kv_store = KvStore::new(cfg)
        .expect("unable to open store");
    let bucket = kv_store.bucket::<String, Json<opts::Task>>(
        Some("tasks")
    ).expect("unable to get bucket");
    let col_bucket = kv_store.bucket::<String, Json<Vec<String>>>(
        Some("tasks")
    ).expect("unable to get bucket");


    let mut store = PersistantStore::new(&bucket);
    let mut col_store = PersistantStore::new(&col_bucket);
    let mut board = Board::new(&mut store, &mut col_store);

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    let default_editor = var("EDITOR")
        .expect("no default editor found");

    let root_file_path = format!(
        "{}{}",
        home_path,
        "/.kanben/files"
    );

    let mut editor = FileEditor::new(
        default_editor,
        root_file_path
    );

    let file_reader = FileReader::new();

    commands::handle(
        opts,
        &mut board,
        &mut writer,
        &mut editor,
        &file_reader
    );
}


