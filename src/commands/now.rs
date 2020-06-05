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
            get_task("task1", Column::Doing),
            get_task("task2", Column::Todo),
            get_task("task3", Column::Done),
            get_task("task4", Column::Done),
            get_task("task5", Column::Done),
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
            get_task("task1", Column::Doing),
            get_task("task2", Column::Todo),
            get_task("task3", Column::Done),
            get_task("task4", Column::Done),
            get_task("task5", Column::Done),
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
            get_task("task1", Column::Doing),
            get_task("task2", Column::Doing),
            get_task("task3", Column::Done),
            get_task("task4", Column::Done),
            get_task("task5", Column::Done),
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
            get_task("task1", Column::Doing),
            get_task("task2", Column::Doing),
            get_task("task3", Column::Done),
            get_task("task4", Column::Done),
            get_task("task5", Column::Done)
        ));


        now(&mut board, &mut writer, true);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        assert_eq!(str_output, "task1,task2".to_string());
    }

    fn get_task(key: &str, column: Column) -> Task {
        Task {
            name: key.to_owned(),
            column,
            description: None,
            tags: None
        }
    }

}

