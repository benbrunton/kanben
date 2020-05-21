use std::io::Write;
use crate::board::BoardAccess;
use crate::opts::Column;

pub fn now<B: BoardAccess>(
    board: &B,
    writer: &mut dyn Write,
    no_newlines: bool
) {

    let delimiter = if no_newlines {
        ","
    } else {
        "\n"
    };

    let tasks = board.get_all_tasks()
        .iter()
        .filter(|item| item.column == Column::Doing)
        .map(|item| {
            item.name.clone()
        }).collect::<Vec<String>>()
        .join(delimiter);

    if tasks != "" {
        let end_char = if no_newlines {
            ""
        } else {
            delimiter
        };
        write!(writer, "{}{}", tasks, end_char).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{opts::Task, test::BoardMock};
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_the_kanban_headers_when_there_are_no_tasks() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();

        now(&mut board, &mut writer, false);

        let output = writer.get_ref();
        assert_eq!(output, b"");
    }

    #[test]
    fn it_outputs_the_inprogress_task_when_it_exists() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();

        board.set_tasks(vec!(
            Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None
            },
            Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None
            },
            Task{
                name: String::from("task3"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task5"),
                column: Column::Done,
                description: None
            },
        ));


        now(&mut board, &mut writer, false);

        let output = writer.get_ref();
        assert_eq!(output, b"task1\n");
    }

    #[test]
    fn it_omits_newlines_when_not_set() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();

        board.set_tasks(vec!(
            Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None
            },
            Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None
            },
            Task{
                name: String::from("task3"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task5"),
                column: Column::Done,
                description: None
            },
        ));


        now(&mut board, &mut writer, true);

        let output = writer.get_ref();
        assert_eq!(output, b"task1");
    }

    #[test]
    fn it_delimits_multiple_tasks_by_newline_by_default() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();

        board.set_tasks(vec!(
            Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None
            },
            Task{
                name: String::from("task2"),
                column: Column::Doing,
                description: None
            },
            Task{
                name: String::from("task3"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task5"),
                column: Column::Done,
                description: None
            },
        ));


        now(&mut board, &mut writer, false);

        let output = writer.get_ref();
        assert_eq!(output, b"task1\ntask2\n");
    }

    #[test]
    fn it_delimits_multiple_tasks_by_comma_when_no_newlines() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();

        board.set_tasks(vec!(
            Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None
            },
            Task{
                name: String::from("task2"),
                column: Column::Doing,
                description: None
            },
            Task{
                name: String::from("task3"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            },
            Task{
                name: String::from("task5"),
                column: Column::Done,
                description: None
            },
        ));


        now(&mut board, &mut writer, true);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        assert_eq!(str_output, "task1,task2".to_string());
    }

}

