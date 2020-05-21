use std::io::Write;
use crate::board::BoardAccess;
use crate::file::Reader;

pub fn view_item<B: BoardAccess>(
    key: String,
    board: &mut B,
    writer: &mut dyn Write,
    reader: &dyn Reader
) {
    let task_result = board.get(&key);

    if task_result.is_none() {
        write!(
            writer,
            "No task named '{}' found.\n",
            &key
        ).unwrap();
        return;
    }

    let task = task_result.unwrap();

    if task.description.is_none() {
        write!(
            writer,
            "Empty description\n",
        ).unwrap();
        return;
    }

    let read_result = reader.read(&task.description.unwrap());

    if read_result.is_none() {
        write!(
            writer,
            "Error loading file for '{}'\n",
            &key
        ).unwrap();
        return;
    }
    write!(writer, "{}\n", read_result.unwrap()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opts::{Task, Column};
    use crate::test::{BoardMock, ReaderMock};
    use std::io::Cursor;

    #[test]
    fn it_outputs_to_writer_from_reader() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut reader = ReaderMock::new();
        let name = "test".to_string();

        let task = Task{
            name: name.clone(),
            column: Column::Todo,
            description: Some("test".to_string())
        };

        board.set(&name, task);
        reader.return_from_read("abcdef");

        view_item(
            name,
            &mut board,
            &mut writer,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"abcdef\n");
    }

    #[test]
    fn it_opens_the_file_in_description() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut reader = ReaderMock::new();
        let name = "test";

        let task = Task{
            name: name.to_string(),
            column: Column::Todo,
            description: Some(name.to_string())
        };

        board.set(name, task);
        reader.return_from_read_when(name, "file contents");

        view_item(
            name.to_string(),
            &mut board,
            &mut writer,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"file contents\n");
    }

    #[test]
    fn it_outputs_a_message_when_no_task_exists() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let reader = ReaderMock::new();
        let name = "test";

        view_item(
            name.to_string(),
            &mut board,
            &mut writer,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"No task named 'test' found.\n");
    }

    #[test]
    fn it_outputs_a_message_when_no_description_exists() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let reader = ReaderMock::new();
        let name = "test";

        let task = Task{
            name: name.to_string(),
            column: Column::Todo,
            description: None
        };

        board.set(name, task);

        view_item(
            name.to_string(),
            &mut board,
            &mut writer,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"Empty description\n");

    }

    #[test]
    fn it_outputs_a_message_when_description_file_fails_to_open() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut reader = ReaderMock::new();
        let name = "test";

        let task = Task{
            name: name.to_string(),
            column: Column::Todo,
            description: Some(name.to_string())
        };

        board.set(name, task);
        reader.return_from_read_when("fakeroute", "file contents");

        view_item(
            name.to_string(),
            &mut board,
            &mut writer,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"Error loading file for 'test'\n");

    }
}
