use std::io::Write;
use math::round;
use crate::board::BoardAccess;
use crate::opts::{Task, Column};

pub fn list_tasks<B: BoardAccess>(
    board: &B, writer: &mut dyn Write
) {
    writer.write(b"TODO:\t\t\tDOING:\t\t\tDONE:\n").unwrap();
    let get_name = |t: &Task| t.name.clone();
    let todo: Vec<String> = board.get_column("todo")
        .iter().map(get_name).collect();
    let doing: Vec<String> = board.get_column("doing")
        .iter().map(get_name).collect();
    let done: Vec<String> = board.get_column("done")
        .iter().map(get_name).collect();

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
    use crate::{test::StoreMock, board::Board};
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_the_kanban_headers_when_there_are_no_tasks() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        list_tasks(&mut board, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"TODO:\t\t\tDOING:\t\t\tDONE:\n\n");
    }

    #[test]
    fn it_sorts_into_rows() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("task1");
        board.create_task("task2");
        board.create_task("task3");
        board.create_task("task4");
        board.create_task("task5");

        board.update("task4", get_task("task4", Column::Done));
        board.update("task1", get_task("task1", Column::Doing));
        board.update("task3", get_task("task3", Column::Doing));


        list_tasks(&mut board, &mut writer);

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
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("task1");
        board.update("task1", get_task("task1", Column::Doing));

        list_tasks(&mut board, &mut writer);

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
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);
        board.create_task("task1-very-long");
        board.create_task("task2-very-long");
        board.create_task("task3");
        board.create_task("task4");
        board.create_task("task5");

        board.update("task1-very-long",
            get_task("task1-very-long", Column::Doing)
        );
        board.update("task3", get_task("task3", Column::Doing));
        board.update("task4", get_task("task4", Column::Done));

        list_tasks(&mut board, &mut writer);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "TODO:\t\t\tDOING:\t\t\tDONE:
task2-very-long\t\ttask1-very-long\t\ttask4
task5\t\t\ttask3\t\t\t\n\n";
        assert_eq!(str_output, expected_output);
    }

    fn get_task(name: &str, column: Column) -> Task {
        Task{
            name: name.to_owned(),
            column,
            description: None
        }
    }
}