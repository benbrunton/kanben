use math::round;
use crate::opts::*;
use crate::store::{Store};
use std::{str, io::Write};

pub fn handle(
    cmd: Option<SubCommand>,
    store: &mut dyn Store,
    writer: &mut dyn Write
) {
    match cmd {
        None => list_tasks(store, writer),
        Some(SubCommand::Add(a)) => add_item(a.title, store),
        Some(SubCommand::Start(a)) => start_item(a.title, store),
        Some(SubCommand::Delete(a)) => delete_item(a.title, store),
        Some(SubCommand::Complete(a)) => complete_item(
            a.title, store
        ),
        Some(SubCommand::ClearDone) => clear_done(store)
    }
}

fn list_tasks(store: &dyn Store, writer: &mut dyn Write) {
    writer.write(b"TODO:\t\t\tDOING:\t\t\tDONE:\n").unwrap();
    let mut todo = vec!();
    let mut doing = vec!();
    let mut done = vec!();

    for value in store.get_all().iter() {
        match value.column {
            Column::Todo => todo.push(value.name.clone()),
            Column::Doing => doing.push(value.name.clone()),
            Column::Done => done.push(value.name.clone()),
        }
    }

    let col_max = find_col_max(vec![
        todo.len(),
        doing.len(),
        done.len()
    ]);

    for n in 0..col_max {
        let next_todo = todo.get(n);
        let next_doing = doing.get(n);
        let next_done = done.get(n);
        write!(writer, "{}", col_text(next_todo)).unwrap();
        write!(writer, "{}", col_text(next_doing)).unwrap();
        if next_done.is_some() {
            write!(writer, "{}", next_done.unwrap()).unwrap();
        }
        write!(writer, "\n").unwrap();
    }
    write!(writer, "\n").unwrap();
}

fn col_text(label: Option<&String>) -> String {
    let mut output = String::from("");
    let col_width = 24;
    let tab_length = 8;
    if label.is_some() {
        output = format!("{}", label.unwrap());
    }

    let tabs_unrounded = (col_width as f64- output.len() as f64)
        / tab_length as f64;
    let tabs = round::ceil(tabs_unrounded, 0) as usize;
    let mut tab_str = String::from("");
    for _ in 0..tabs {
       tab_str = format!("{}{}", tab_str, "\t"); 
    }

    format!("{}{}", output, tab_str)
}

fn add_item(name: String, store: &mut dyn Store) {
    if is_valid_key(&name) {
        let new_item = Task {
            name: String::from(&name),
            column: Column::Todo,
            description: String::from("")   
        };
        store.set(&name, new_item);
    }
}

fn is_valid_key(name: &str) -> bool {
    name.trim().len() > 0
}

fn start_item(name: String, store: &mut dyn Store) {
    move_item(name, store, Column::Doing);
}

fn complete_item(name: String, store: &mut dyn Store) {
    move_item(name, store, Column::Done);
}

fn move_item(name: String, store: &mut dyn Store, column: Column) {
    let mut item = store.get(&name).unwrap();
    item.column = column;
    store.set(&name, item);
}

fn delete_item(name: String, store: &mut dyn Store) {
    store.rm(&name);
}

fn clear_done(store: &mut dyn Store) {
    store.get_all().iter().filter(|task| {
        task.column == Column::Done
    }).for_each(|task| store.rm(&task.name));
}

fn find_col_max(cols: Vec<usize>) -> usize {
    *cols.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::StoreMock;
    use std::io::Cursor;

    #[test]
    fn it_adds_a_new_item_to_the_store() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let task = Task {
            name: name.clone(),
            column: Column::Todo,
            description: String::from("") 
        };

        let cmd = SubCommand::Add(item.clone());
        handle(Some(cmd), &mut store, &mut writer);
        assert!(store.set_called_with(&name, &task));
    }

    #[test]
    fn it_doesnt_create_a_new_item_for_a_blank_key() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let name = String::from(" ");
        let item = Item{
            title: name.clone()
        };

        let cmd = SubCommand::Add(item.clone());
        handle(Some(cmd), &mut store, &mut writer);
        assert!(!store.set_called());
    }

    #[test]
    fn it_outputs_the_kanban_headers_when_there_are_no_tasks() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        handle(None, &mut store, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"TODO:\t\t\tDOING:\t\t\tDONE:\n\n");
    }

    #[test]
    fn it_displays_any_tasks_on_the_board() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: String::from("") 
            }),
        ));

        handle(None, &mut store, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
\t\t\ttask1\t\t\t\n\n";
        assert_eq!(str_output, expected_output);
    }

    #[test]
    fn it_sorts_into_rows() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: String::from("") 
            }),
            ("task2", Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: String::from("") 
            }),
            ("task3", Task{
                name: String::from("task3"),
                column: Column::Doing,
                description: String::from("") 
            }),
            ("task4", Task{
                name: String::from("task4"),
                column: Column::Done,
                description: String::from("") 
            }),
            ("task5", Task{
                name: String::from("task5"),
                column: Column::Todo,
                description: String::from("") 
            }),
        ));

        handle(None, &mut store, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
task2\t\t\ttask1\t\t\ttask4
task5\t\t\ttask3\t\t\t\n\n";
        assert_eq!(str_output, expected_output);

    }

    #[test]
    fn it_can_delete_an_item() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let cmd = SubCommand::Delete(item.clone());
        handle(Some(cmd), &mut store, &mut writer);
        assert!(store.rm_called_with(&name));

    }

    #[test]
    fn it_can_clear_done_column() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: String::from("") 
            }),
            ("task2", Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: String::from("") 
            }),
            ("task3", Task{
                name: String::from("task3"),
                column: Column::Done,
                description: String::from("") 
            }),
            ("task4", Task{
                name: String::from("task4"),
                column: Column::Done,
                description: String::from("") 
            }),
            ("task5", Task{
                name: String::from("task5"),
                column: Column::Done,
                description: String::from("") 
            }),
        ));

        handle(
            Some(SubCommand::ClearDone),
            &mut store,
            &mut writer
        );

        assert!(store.rm_called_with("task3"));
        assert!(store.rm_called_with("task4"));
        assert!(store.rm_called_with("task5"));
        assert!(!store.rm_called_with("task1"));
    }

    #[test]
    fn it_takes_long_names_off_the_tabs() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1-very-long"),
                column: Column::Doing,
                description: String::from("") 
            }),
            ("task2-very-long", Task{
                name: String::from("task2-very-long"),
                column: Column::Todo,
                description: String::from("") 
            }),
            ("task3", Task{
                name: String::from("task3"),
                column: Column::Doing,
                description: String::from("") 
            }),
            ("task4", Task{
                name: String::from("task4"),
                column: Column::Done,
                description: String::from("") 
            }),
            ("task5", Task{
                name: String::from("task5"),
                column: Column::Todo,
                description: String::from("") 
            }),
        ));

        handle(None, &mut store, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
task2-very-long\t\ttask1-very-long\t\ttask4
task5\t\t\ttask3\t\t\t\n\n";
        assert_eq!(str_output, expected_output);

    }

}
