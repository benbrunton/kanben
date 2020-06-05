use std::io::Write;
use crate::board::BoardAccess;
use crate::file::Reader;
use colored::*;

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

    if task.tags.is_some() {
        let tags = task.tags.unwrap();
        let tag_str = format!("tags: {}\n", tags.join(", "));
        write!(
            writer,
            "{}",
            tag_str.bold()
        ).unwrap();
    }

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
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_to_writer_from_reader() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut reader = ReaderMock::new();
        let name = "test".to_string();

        let mut task = get_task(&name, Column::Todo);
        task.description = Some("test".to_owned());

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

        let mut task = get_task(name, Column::Todo);
        task.description = Some(name.to_string());

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

        let task = get_task(name, Column::Todo);

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

        let mut task = get_task(name, Column::Todo);
        task.description = Some(name.to_string());

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

    #[test]
    fn it_outputs_tags_at_the_top_of_a_message() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut reader = ReaderMock::new();
        let name = "test";

        let mut task = get_task(name, Column::Todo);
        task.description = Some(name.to_owned());
        task.tags = Some(vec!("tag1".to_owned()));

        board.set(name, task);
        reader.return_from_read_when(name, "file contents");

        view_item(
            name.to_string(),
            &mut board,
            &mut writer,
            &reader
        );
        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected = format!(
            "{}{}", 
            "tags: tag1\n".bold(),
            "file contents\n"
        );
        assert_eq!(str_output, &expected);

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
