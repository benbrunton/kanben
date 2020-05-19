use clap::{App, Clap};

mod commands;
mod opts;
mod store;

#[cfg(test)]
mod test;

use opts::Opts;
use store::PersistantStore;

fn main() {
    let _app = App::new("kanben");
    let opts: Opts = Opts::parse();
    let mut store = PersistantStore::new();
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    commands::handle(opts.subcmd, &mut store, &mut writer);
}


