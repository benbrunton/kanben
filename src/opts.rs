use clap::Clap;
use serde::{Serialize, Deserialize};

#[derive(Clap)]
#[clap(version = "1.0", author = "Ben Brunton")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
    #[clap(short, long)]
    pub no_newlines: bool
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about="Adds a new task in todo column")]
    Add(Item),
    #[clap(about="Move a task into doing")]
    Start(Item),
    #[clap(about="Move a task into done")]
    Complete(Item),
    #[clap(about="Delete a task")]
    Delete(Item),
    #[clap(about="clear tasks from the done column")]
    ClearDone,
    #[clap(about="Edit the information about a task")]
    Edit(Item),
    #[clap(about="View any additional information about a task")]
    View(Item),
    #[clap(about="Outputs in-progress tasks")]
    Now,
    #[clap(about="Re-indexes columns")]
    Reindex
}

#[derive(Clap, Clone, PartialEq)]
pub struct Item {
    pub title: String
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Task {
    pub name: String,
    pub column: Column,
    pub description: Option<String> 
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Column {
    Todo,
    Doing,
    Done
}
