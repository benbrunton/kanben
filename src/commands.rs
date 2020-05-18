use crate::opts::*;
use kv::*;
use dirs::home_dir;
use std::cmp;

pub fn handle(cmd: Option<SubCommand>) {

    let home_path_bfr = home_dir().unwrap();
    let home_path = home_path_bfr.to_str().unwrap();
    let cfg_location = format!("{}{}", home_path, "/.kanben");
    let mut cfg = Config::new(&cfg_location);

    let store = Store::new(cfg).expect("unable to open store");
    let bucket = store.bucket::<String, Json<Task>>(Some("tasks"))
        .expect("unable to get bucket");

    match cmd {
        None => list_tasks(bucket),
        Some(SubCommand::Add(a)) => add_item(a.title, bucket),
        Some(SubCommand::Start(a)) => start_item(a.title, bucket),
        Some(SubCommand::Complete(a)) => complete_item(a.title, bucket),
    }

}

fn list_tasks(task_bucket: Bucket<String, Json<Task>>) {
    println!("KANBEN!");
    println!("TODO:\t\tDOING:\t\tDONE:");
    let mut todo = vec!();
    let mut doing = vec!();
    let mut done = vec!();

    for wrapped_task in task_bucket.iter() {
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

fn add_item(name: String, task_bucket: Bucket<String, Json<Task>>) {
    let new_item = Task {
        name: String::from(&name),
        column: Column::Todo,
        description: String::from("")   
    };
    task_bucket.set(name.clone(), Json(new_item));
}

fn start_item(name: String, task_bucket: Bucket<String, Json<Task>>) {
    let retrieve_result = task_bucket.get(name.clone());
    let item_result = retrieve_result.expect("unable to retrieve");
    let json_item = item_result.expect("unable to get item"); 
    let mut item = json_item.as_ref().clone();
    item.column = Column::Doing;
    task_bucket.set(name.clone(), Json(item));
}

fn complete_item(name: String, task_bucket: Bucket<String, Json<Task>>) {
    let retrieve_result = task_bucket.get(name.clone());
    let item_result = retrieve_result.expect("unable to retrieve");
    let json_item = item_result.expect("unable to get item"); 
    let mut item = json_item.as_ref().clone();
    item.column = Column::Done;
    task_bucket.set(name.clone(), Json(item));
}

