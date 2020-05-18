use clap::Clap;
use serde::{Serialize, Deserialize};

#[derive(Clap)]
#[clap(version = "1.0", author = "Ben Brunton")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Clap)]
pub enum SubCommand {
    Add(Item),
    Start(Item),
    Complete(Item)
}

#[derive(Clap)]
pub struct Item {
    pub title: String
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Task {
    pub name: String,
    pub column: Column,
    pub description: String 
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Column {
    Todo,
    Doing,
    Done
}

