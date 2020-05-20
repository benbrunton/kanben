use crate::opts::{Task, Column};
use crate::store::Store;

pub fn add_item(name: String, store: &mut dyn Store) {
    if is_valid_key(&name) {
        let new_item = Task {
            name: String::from(&name),
            column: Column::Todo,
            description: None
        };
        store.set(&name, new_item);
    }
}

fn is_valid_key(name: &str) -> bool {
    name.trim().len() > 0
}

pub fn start_item(name: String, store: &mut dyn Store) {
    move_item(name, store, Column::Doing);
}

pub fn complete_item(name: String, store: &mut dyn Store) {
    move_item(name, store, Column::Done);
}

fn move_item(name: String, store: &mut dyn Store, column: Column) {
    let mut item = store.get(&name).unwrap();
    item.column = column;
    store.set(&name, item);
}

pub fn delete_item(name: String, store: &mut dyn Store) {
    store.rm(&name);
}

pub fn clear_done(store: &mut dyn Store) {
    store.get_all().iter().filter(|task| {
        task.column == Column::Done
    }).for_each(|task| store.rm(&task.name));
}
