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
    Complete(Item),
    Delete(Item),
    ClearDone
}

#[derive(Clap, Clone, PartialEq)]
pub struct Item {
    pub title: String
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Task {
    pub name: String,
    pub column: Column,
    pub description: String 
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Column {
    Todo,
    Doing,
    Done
}

