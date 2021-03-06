use std::io::Write;
use crate::board::BoardAccess;
use crate::editor::Editor;

// if task has an associated file, open that
// otherwise create one and open it

pub fn edit_item<B: BoardAccess>(
    key: String,
    board: &mut B,
    editor: &mut dyn Editor,
    writer: &mut dyn Write
) {
    let task_result = board.get(&key);

    if task_result.is_none() {
        write!(writer, "No item named '{}' found.\n", &key).unwrap();
        return;
    }

    let mut task = task_result.unwrap();
    if task.description.is_some() {
        let description = task.description.unwrap();
        if description.trim() != "".to_string() {
            editor.open(&description);
            return;
        }
    }

    let result = editor.create(&key);
    task.description = Some(result.unwrap().clone());
    board.update(&key, task);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opts::{Task, Column};
    use crate::test::{BoardMock, EditorMock};
    use std::{str, io::Cursor};

    #[test]
    fn it_opens_an_associated_file_when_there_is_one() {
        let key = "test".to_owned();
        let path_to_file = "path to test file".to_owned();
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());

        let mut task = get_task(&key, Column::Todo);
        task.description = Some(path_to_file.clone());

        board.set(&key, task);
        
        edit_item(key, &mut board, &mut editor, &mut writer);

        assert!(editor.open_called_with(&path_to_file));
    }

    #[test]
    fn it_creates_a_new_file_when_there_is_none() {
        let key = String::from("test");
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        let task = get_task(&key, Column::Todo);

        board.set(&key, task);
        edit_item(key.clone(), &mut board, &mut editor, &mut writer);

        assert!(editor.create_called_with(&key));
    }

    #[test]
    fn it_outputs_a_message_when_there_is_no_task() {
        let key = String::from("test");
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        
        edit_item(key.clone(), &mut board, &mut editor, &mut writer);
        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "No item named 'test' found.\n";

        assert_eq!(str_output, expected_output);
    }

    #[test]
    fn it_stores_the_path_to_a_task_when_a_file_is_created() {
        let key = "test".to_owned();
        let filepath = "test path".to_owned();
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        let task = get_task(&key, Column::Todo);

        let mut new_task = get_task(&key, Column::Todo);
        new_task.description = Some(filepath.clone());

        board.set(&key, task);
        editor.return_from_create(Ok(filepath.clone()));

        edit_item(
            key.clone(),
            &mut board,
            &mut editor,
            &mut writer
        );

        assert!(board.update_called_with(&key, &new_task));
    }

    #[test]
    fn when_a_path_is_empty_string_it_creates_a_new_file() {
        let key = "test".to_owned();
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        let mut task = get_task(&key, Column::Todo);
        task.description = Some("".to_owned());

        board.set(&key, task);
        edit_item(
            key.clone(),
            &mut board,
            &mut editor,
            &mut writer
        );

        assert!(editor.create_called_with(&key));
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
