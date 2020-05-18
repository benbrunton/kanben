use clap::{App, Clap};

mod commands;
mod opts;

use opts::Opts;

fn main() {
    let mut app = App::new("kanben");
    let opts: Opts = Opts::parse();

    commands::handle(opts.subcmd);
}


