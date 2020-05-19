use crate::opts::*;
use crate::store::Store;
use kv::Json;
use std::cmp;

pub fn handle(cmd: Option<SubCommand>, store: &mut Store) {
    match cmd {
        None => list_tasks(store),
        Some(SubCommand::Add(a)) => add_item(a.title, store),
        Some(SubCommand::Start(a)) => start_item(a.title, store),
        Some(SubCommand::Complete(a)) => complete_item(a.title, store),
    }

}

fn list_tasks(store: &Store) {
    println!("KANBEN!");
    println!("TODO:\t\tDOING:\t\tDONE:");
    let mut todo = vec!();
    let mut doing = vec!();
    let mut done = vec!();

    for wrapped_task in store.get_all() {
        let task = wrapped_task.unwrap();
        let json_value = task.value::<Json<Task>>().unwrap();
        let value = json_value.as_ref();
        match value.column {
            Column::Todo => todo.push(value.name.clone()),
            Column::Doing => doing.push(value.name.clone()),
            Column::Done => done.push(value.name.clone()),
        }
    }

    let row_max = cmp::max(
        cmp::max(todo.len(), doing.len()),
        done.len()
    );

    for n in 0..row_max {
        let next_todo = todo.get(n);
        let next_doing = doing.get(n);
        let next_done = done.get(n);
        if next_todo.is_some() {
            print!("{}", next_todo.unwrap());
        }
        print!("\t\t");
        if next_doing.is_some() {
            print!("{}", next_doing.unwrap());
        }
        print!("\t\t");
        if next_done.is_some() {
            print!("{}", next_done.unwrap());
        }
        println!("");
    }
    println!("");

}

fn add_item(name: String, store: &mut Store) {
    let new_item = Task {
        name: String::from(&name),
        column: Column::Todo,
        description: String::from("")   
    };
    store.set(&name, new_item);
}

fn start_item(name: String, store: &mut Store) {
    let item_result = store.get(&name);
    let json_item = item_result.expect("unable to get item"); 
    let mut item = json_item.as_ref().clone();
    item.column = Column::Doing;
    store.set(&name, item);
}

fn complete_item(name: String, store: &mut Store) {
    let item_result = store.get(&name);
    let json_item = item_result.expect("unable to get item"); 
    let mut item = json_item.as_ref().clone();
    item.column = Column::Done;
    store.set(&name, item);
}

