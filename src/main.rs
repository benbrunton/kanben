use std::env::var;
use dirs::home_dir;
use clap::{App, Clap};

mod commands;
mod opts;
mod store;
mod editor;
mod file;

#[cfg(test)]
mod test;

use opts::Opts;
use store::PersistantStore;
use editor::FileEditor;
use file::FileReader;

fn main() {
    let _app = App::new("kanben");

    let opts: Opts = Opts::parse();

    let mut store = PersistantStore::new();

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    let default_editor = var("EDITOR")
        .expect("no default editor found");

    let home_path_bfr = home_dir().unwrap();
    let home_path = home_path_bfr.to_str().unwrap();
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
        opts.subcmd,
        &mut store,
        &mut writer,
        &mut editor,
        &file_reader
    );
}


