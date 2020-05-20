use std::io::Write;
use math::round;
use crate::store::Store;
use crate::opts::Column;

pub fn list_tasks(store: &dyn Store, writer: &mut dyn Write) {
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

fn find_col_max(cols: Vec<usize>) -> usize {
    *cols.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{opts::Task, test::StoreMock};
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_the_kanban_headers_when_there_are_no_tasks() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        list_tasks(&mut store, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"TODO:\t\t\tDOING:\t\t\tDONE:\n\n");
    }

    #[test]
    fn it_sorts_into_rows() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None,
            }),
            ("task2", Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None,
            }),
            ("task3", Task{
                name: String::from("task3"),
                column: Column::Doing,
                description: None
            }),
            ("task4", Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            }),
            ("task5", Task{
                name: String::from("task5"),
                column: Column::Todo,
                description: None
            }),
        ));

        list_tasks(&mut store, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
task2\t\t\ttask1\t\t\ttask4
task5\t\t\ttask3\t\t\t\n\n";
        assert_eq!(str_output, expected_output);
    }

    #[test]
    fn it_displays_any_tasks_on_the_board() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None
            }),
        ));

        list_tasks(&mut store, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
\t\t\ttask1\t\t\t\n\n";
        assert_eq!(str_output, expected_output);
    }

    #[test]
    fn it_takes_long_names_off_the_tabs() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1-very-long"),
                column: Column::Doing,
                description: None
            }),
            ("task2-very-long", Task{
                name: String::from("task2-very-long"),
                column: Column::Todo,
                description: None
            }),
            ("task3", Task{
                name: String::from("task3"),
                column: Column::Doing,
                description: None
            }),
            ("task4", Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            }),
            ("task5", Task{
                name: String::from("task5"),
                column: Column::Todo,
                description: None
            }),
        ));

        list_tasks(&mut store, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
task2-very-long\t\ttask1-very-long\t\ttask4
task5\t\t\ttask3\t\t\t\n\n";
        assert_eq!(str_output, expected_output);
    }
}
