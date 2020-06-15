use std::env::var;
use dirs::home_dir;
use clap::{App, Clap};
use colored::*;
use reqwest;

use kv::{Store as KvStore, Config, Json};

use flexi_logger::{Logger, opt_format, Duplicate};
use log::info;

mod commands;
mod opts;
mod store;
mod editor;
mod file;
mod board;
mod web;
mod archive;

#[cfg(test)]
mod test;

use opts::Opts;
use store::PersistantStore;
use editor::FileEditor;
use file::FileReader;
use archive::ZipArchive;
use board::Board;
use web::{Client, WebClient};

fn main() {
//    let web_service_path = env!("KANBEN_WEB_SERVICE_PATH");


    let _app = App::new("kanben");

    let opts: Opts = Opts::parse();

    let log_handle = Logger::with_env_or_str(
        "info"
    ).log_to_file()
        .directory("/tmp/.kanben")
        .format(opt_format);

    if opts.verbose {
        log_handle.duplicate_to_stderr(Duplicate::Info)
            .start().unwrap();
    } else {
        log_handle.start()
            .unwrap();
    }

    info!("kanben starting up!");


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
        Some("columns")
    ).expect("unable to get bucket");
    let tag_bucket = kv_store.bucket::<String, Json<Vec<String>>>(
        Some("tags")
    ).expect("unable to get bucket");


    let mut store = PersistantStore::new(&bucket);
    let mut col_store = PersistantStore::new(&col_bucket);
    let mut tag_store = PersistantStore::new(&tag_bucket);
    let mut board = Board::new(
        &mut store,
        &mut col_store,
        &mut tag_store
    );

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    let default_editor_result = var("EDITOR");

    let default_editor = if default_editor_result.is_err() {
        println!("{}", "No default editor found.".red());
        println!("{}",
            "Set EDITOR environment variable to enable `kanben edit`.".red());
        None
    } else {
        Some(default_editor_result.unwrap())
    };

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
    
    let client = reqwest::blocking::Client::new();
    let http_client = Client::new(client);
    let mut web = WebClient::new(http_client);
    let archive = ZipArchive::new(&cfg_location);

    commands::handle(
        opts,
        &mut board,
        &mut writer,
        &mut editor,
        &file_reader,
        &mut web,
        &archive
    );
}


