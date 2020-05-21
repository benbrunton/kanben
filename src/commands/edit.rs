use std::io::Write;
use crate::store::Store;
use crate::editor::Editor;

// if task has an associated file, open that
// otherwise create one and open it

pub fn edit_item(
    key: String,
    store: &mut dyn Store,
    editor: &mut dyn Editor,
    writer: &mut dyn Write
) {
    let task_result = store.get(&key);

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
    store.set(&key, task);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opts::{Task, Column};
    use crate::test::{StoreMock, EditorMock};
    use std::{str, io::Cursor};

    #[test]
    fn it_opens_an_associated_file_when_there_is_one() {
        let key = String::from("test");
        let path_to_file = String::from("path to test file");
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());

        let task = Task{
            name: key.clone(),
            column: Column::Todo,
            description: Some(path_to_file.clone())
        };

        store.return_from_get(task);
        
        edit_item(key, &mut store, &mut editor, &mut writer);

        assert!(editor.open_called_with(&path_to_file));
    }

    #[test]
    fn it_creates_a_new_file_when_there_is_none() {
        let key = String::from("test");
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        let task = Task{
            name: key.clone(),
            column: Column::Todo,
            description: None
        };

        store.return_from_get(task);
        edit_item(key.clone(), &mut store, &mut editor, &mut writer);

        assert!(editor.create_called_with(&key));
    }

    #[test]
    fn it_outputs_a_message_when_there_is_no_task() {
        let key = String::from("test");
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        
        edit_item(key.clone(), &mut store, &mut editor, &mut writer);
        let output = writer.get_ref();
        let str_output = str::from_utf8(&output).unwrap();
        let expected_output = "No item named 'test' found.\n";

        assert_eq!(str_output, expected_output);
    }

    #[test]
    fn it_stores_the_path_to_a_task_when_a_file_is_created() {
        let key = "test".to_string();
        let filepath = "test path".to_string();
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        let task = Task{
            name: key.clone(),
            column: Column::Todo,
            description: None
        };

        let new_task = Task{
            name: key.clone(),
            column: Column::Todo,
            description: Some(filepath.clone())
        };


        store.return_from_get(task);
        editor.return_from_create(Ok(filepath.clone()));

        edit_item(
            key.clone(),
            &mut store,
            &mut editor,
            &mut writer
        );

        assert!(store.set_called_with(&key, &new_task));
    }

    #[test]
    fn when_a_path_is_empty_string_it_creates_a_new_file() {
        let key = String::from("test");
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let mut writer = Cursor::new(vec!());
        let task = Task{
            name: key.clone(),
            column: Column::Todo,
            description: Some("".to_string())
        };

        store.return_from_get(task);
        edit_item(
            key.clone(),
            &mut store,
            &mut editor,
            &mut writer
        );

        assert!(editor.create_called_with(&key));
    }
}
