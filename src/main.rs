use clap::{App, Clap};

mod commands;
mod opts;
mod store;

use opts::Opts;
use store::Store;

fn main() {
    let _app = App::new("kanben");
    let opts: Opts = Opts::parse();
    let mut store = Store::new();

    commands::handle(opts.subcmd, &mut store);
}


