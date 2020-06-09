use std::io::Write;
use crate::board::BoardAccess;
use crate::opts::Column;

pub fn now<B: BoardAccess>(
    board: &B,
    writer: &mut dyn Write,
    no_newlines: bool,
    tag: Option<String>
) {

    let delimiter = if no_newlines {
        ","
    } else {
        "\n"
    };

    let tasks = board.get_column("doing", tag)
        .iter()
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
    use crate::opts::Task;
    use crate::test::StoreMock;
    use crate::board::Board;
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_the_kanban_headers_when_there_are_no_tasks() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        now(&mut board, &mut writer, false, None);

        let output = writer.get_ref();
        assert_eq!(output, b"");
    }

    #[test]
    fn it_outputs_the_inprogress_task_when_it_exists() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("task1", None);
        board.create_task("task2", None);
        board.create_task("task3", None);
        board.create_task("task4", None);
        board.create_task("task5", None);

        board.update("task4", get_task("task4", Column::Done));
        board.update("task1", get_task("task1", Column::Doing));
        board.update("task3", get_task("task3", Column::Done));


        now(&mut board, &mut writer, false, None);

        let output = writer.get_ref();
        assert_eq!(output, b"task1\n");
    }

    #[test]
    fn it_omits_newlines_when_not_set() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("task1", None);
        board.create_task("task2", None);
        board.create_task("task3", None);
        board.create_task("task4", None);
        board.create_task("task5", None);

        board.update("task4", get_task("task4", Column::Done));
        board.update("task1", get_task("task1", Column::Doing));
        board.update("task3", get_task("task3", Column::Done));

        now(&mut board, &mut writer, true, None);

        let output = writer.get_ref();
        assert_eq!(output, b"task1");
    }

    #[test]
    fn it_delimits_multiple_tasks_by_newline_by_default() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("task1", None);
        board.create_task("task2", None);
        board.create_task("task3", None);
        board.create_task("task4", None);
        board.create_task("task5", None);

        board.update("task1", get_task("task1", Column::Doing));
        board.update("task2", get_task("task2", Column::Doing));
        board.update("task3", get_task("task3", Column::Done));

        now(&mut board, &mut writer, false, None);

        let output = writer.get_ref();
        assert_eq!(output, b"task1\ntask2\n");
    }

    #[test]
    fn it_delimits_multiple_tasks_by_comma_when_no_newlines() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("task1", None);
        board.create_task("task2", None);
        board.create_task("task3", None);
        board.create_task("task4", None);
        board.create_task("task5", None);

        board.update("task1", get_task("task1", Column::Doing));
        board.update("task2", get_task("task2", Column::Doing));
        board.update("task3", get_task("task3", Column::Done));
       
        now(&mut board, &mut writer, true, None);

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        assert_eq!(str_output, "task1,task2".to_string());
    }

    #[test]
    fn it_filters_by_tag() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("task1", None);
        board.create_task("task2", None);
        board.create_task("task3", None);
        board.create_task("task4", None);
        board.create_task("task5", None);

        let mut task1 = get_task("task1", Column::Doing);
        task1.tags = Some(vec!("tag".to_owned()));
        board.update("task1", task1);
        board.update("task2", get_task("task2", Column::Doing));
        board.update("task3", get_task("task3", Column::Done));
       
        now(
            &mut board,
            &mut writer,
            false,
            Some("tag".to_owned()));

        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        assert_eq!(str_output, "task1\n".to_string());

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

