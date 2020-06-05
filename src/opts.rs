use clap::Clap;
use serde::{Serialize, Deserialize};

#[derive(Clap)]
#[clap(version = "1.0", author = "Ben Brunton")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
    #[clap(short, long, about="supress newlines in output")]
    pub no_newlines: bool,
    #[clap(short, long, about="filter by tags")]
    pub tag: Option<String>
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about="Adds a new task in todo column")]
    Add(NewItem),
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
    Reindex,
    #[clap(about="moves task to top of priorities")]
    Top(Item),
    #[clap(about="add a tag to a task or view a tasks tags")]
    Tag(TagItem)
}

#[derive(Clap, Clone, PartialEq)]
pub struct Item {
    #[clap(about="Name of task")]
    pub title: String
}

#[derive(Clap, Clone, PartialEq)]
pub struct NewItem {
    #[clap(about="Name of task")]
    pub title: String,
    #[clap(short, long, about="add a tag to the new task")]
    pub tag: Option<String>
}

#[derive(Clap, Clone, PartialEq)]
pub struct TagItem {
    #[clap(about="Name of task")]
    pub title: String,
    #[clap(about="Name of tag")]
    pub tag: Option<String>,
    #[clap(short, long, about="remove tag")]
    pub remove: bool
}


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Task {
    pub name: String,
    pub column: Column,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Column {
    Todo,
    Doing,
    Done
}
