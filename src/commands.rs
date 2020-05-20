use crate::opts::*;
use crate::store::Store;
use crate::editor::Editor;
use crate::file::Reader;
use std::io::Write;

mod list;
mod edit;
mod view;
use list::list_tasks;
use edit::edit_item;
use view::view_item;

pub fn handle(
    cmd: Option<SubCommand>,
    store: &mut dyn Store,
    writer: &mut dyn Write,
    editor: &mut dyn Editor,
    file_reader: &dyn Reader,
) {
    match cmd {
        None => list_tasks(store, writer),
        Some(SubCommand::Add(a)) => add_item(a.title, store),
        Some(SubCommand::Start(a)) => start_item(a.title, store),
        Some(SubCommand::Delete(a)) => delete_item(a.title, store),
        Some(SubCommand::Edit(a)) => edit_item(
            a.title, store, editor, writer
        ),
        Some(SubCommand::View(a)) => view_item(
            a.title, store, writer, file_reader
        ),
        Some(SubCommand::Complete(a)) => complete_item(
            a.title, store
        ),
        Some(SubCommand::ClearDone) => clear_done(store)
    }
}

fn add_item(name: String, store: &mut dyn Store) {
    if is_valid_key(&name) {
        let new_item = Task {
            name: String::from(&name),
            column: Column::Todo,
            description: None
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{StoreMock, EditorMock, ReaderMock};
    use std::io::Cursor;

    #[test]
    fn it_adds_a_new_item_to_the_store() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let task = Task {
            name: name.clone(),
            column: Column::Todo,
            description: None
        };

        let cmd = SubCommand::Add(item.clone());
        handle(
            Some(cmd),
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(store.set_called_with(&name, &task));
    }

    #[test]
    fn it_doesnt_create_a_new_item_for_a_blank_key() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from(" ");
        let item = Item{
            title: name.clone()
        };

        let cmd = SubCommand::Add(item.clone());
        handle(
            Some(cmd),
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(!store.set_called());
    }

    #[test]
    fn it_lists_tasks_when_no_command_is_passed() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();

        handle(
            None,
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"TODO:\t\t\tDOING:\t\t\tDONE:\n\n");
    }

    #[test]
    fn it_can_delete_an_item() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let cmd = SubCommand::Delete(item.clone());
        handle(
            Some(cmd),
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(store.rm_called_with(&name));

    }

    #[test]
    fn it_can_clear_done_column() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();

        store.insert_tasks(vec!(
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None
            }),
            ("task2", Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None
            }),
            ("task3", Task{
                name: String::from("task3"),
                column: Column::Done,
                description: None
            }),
            ("task4", Task{
                name: String::from("task4"),
                column: Column::Done,
                description: None
            }),
            ("task5", Task{
                name: String::from("task5"),
                column: Column::Done,
                description: None
            }),
        ));

        handle(
            Some(SubCommand::ClearDone),
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );

        assert!(store.rm_called_with("task3"));
        assert!(store.rm_called_with("task4"));
        assert!(store.rm_called_with("task5"));
        assert!(!store.rm_called_with("task1"));
    }

    #[test]
    fn it_opens_an_editor_when_edit_command_is_passed() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let task = Task{
            name: name.clone(),
            column: Column::Todo,
            description: Some("test".to_string())
        };

        store.return_from_get(task);


        let cmd = SubCommand::Edit(item.clone());
        handle(
            Some(cmd),
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(editor.open_called());
    }

    #[test]
    fn it_outputs_to_stdout_when_viewing_description() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut editor = EditorMock::new();
        let mut reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let task = Task{
            name: name.clone(),
            column: Column::Todo,
            description: Some("test".to_string())
        };

        store.return_from_get(task);
        reader.return_from_read("abcdef");

        let cmd = SubCommand::View(item.clone());
        handle(
            Some(cmd),
            &mut store,
            &mut writer,
            &mut editor,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"abcdef\n");
    }

}
