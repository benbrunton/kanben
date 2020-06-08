use crate::opts::*;
use crate::board::BoardAccess;
use crate::editor::Editor;
use crate::file::Reader;
use std::io::Write;

mod list;
mod edit;
mod view;
mod standard_actions;
mod now;
mod reindex;
mod tag;
use list::{list_all, list_tasks};
use edit::edit_item;
use view::view_item;
use now::now;
use reindex::reindex;
use standard_actions::{
    add_item,
    start_item,
    delete_item,
    clear_done,
    complete_item,
    top
};
use tag::tag;

pub fn handle<B: BoardAccess, W: Write>(
    opts: Opts,
    board: &mut B,
    writer: &mut W,
    editor: &mut dyn Editor,
    file_reader: &dyn Reader,
) {
    match opts.subcmd {
        None => list_tasks(opts.tag, board, writer),
        Some(SubCommand::Add(a)) => add_item(
            a.title, a.tag, board
        ),
        Some(SubCommand::Start(a)) => start_item(a.title, board),
        Some(SubCommand::Delete(a)) => delete_item(a.title, board),
        Some(SubCommand::Edit(a)) => edit_item(
            a.title, board, editor, writer
        ),
        Some(SubCommand::View(a)) => view_item(
            a.title, board, writer, file_reader
        ),
        Some(SubCommand::Complete(a)) => complete_item(
            a.title, board
        ),
        Some(SubCommand::ClearDone) => clear_done(board),
        Some(SubCommand::Now) => now(
            board, writer, opts.no_newlines
        ),
        Some(SubCommand::Reindex) => reindex(
            board, writer
        ),
        Some(SubCommand::Top(a)) => top(a.title, board),
        Some(SubCommand::Tag(a)) => tag(
            &a.title, a.tag, a.remove, board, writer
        ),
        Some(SubCommand::Tasks) => list_all(opts.tag, board, writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{BoardMock, EditorMock, ReaderMock};
    use std::io::Cursor;

    #[test]
    fn it_adds_a_new_item_to_the_store() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from("test");
        let item = NewItem{
            title: name.clone(),
            tag: None
        };

        let opts = Opts {
            subcmd: Some(SubCommand::Add(item.clone())),
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(board.create_task_called_with(&name));
    }

    #[test]
    fn it_doesnt_create_a_new_item_for_a_blank_key() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from(" ");
        let item = NewItem{
            title: name.clone(),
            tag: None
        };

        let opts = Opts {
            subcmd: Some(SubCommand::Add(item.clone())),
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(!board.create_task_called_with(" "));
    }

    #[test]
    fn it_lists_tasks_when_no_command_is_passed() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let opts = Opts {
            subcmd: None,
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
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
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let opts = Opts {
            subcmd: Some(SubCommand::Delete(item.clone())),
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(board.remove_called_with(&name));

    }

    #[test]
    fn it_can_clear_done_column() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();

        board.set_tasks(vec!(
            get_task("task1", Column::Doing),
            get_task("task2", Column::Todo),
            get_task("task3", Column::Done),
            get_task("task4", Column::Done),
            get_task("task5", Column::Done),
        ));

        let opts = Opts {
            subcmd: Some(SubCommand::ClearDone),
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );

        assert!(board.remove_called_with("task3"));
        assert!(board.remove_called_with("task4"));
        assert!(board.remove_called_with("task5"));
        assert!(!board.remove_called_with("task1"));
    }

    #[test]
    fn it_opens_an_editor_when_edit_command_is_passed() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let mut task = get_task(&name, Column::Todo);
        task.description = Some("test".to_owned());

        board.set(&name, task);

        let opts = Opts {
            subcmd: Some(SubCommand::Edit(item.clone())),
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );
        assert!(editor.open_called());
    }

    #[test]
    fn it_outputs_to_stdout_when_viewing_description() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let mut reader = ReaderMock::new();
        let name = String::from("test");
        let item = Item{
            title: name.clone()
        };

        let mut task = get_task(&name, Column::Todo);
        task.description = Some("test".to_owned());

        board.set(&name, task);
        reader.return_from_read("abcdef");

        let opts = Opts{
            subcmd: Some(SubCommand::View(item.clone())),
            no_newlines: false,
            tag: None
        };
        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"abcdef\n");
    }

    #[test]
    fn it_outputs_nothing_when_there_are_no_tasks_for_now() {
        let mut writer = Cursor::new(vec!());
        let mut board = BoardMock::new();
        let mut editor = EditorMock::new();
        let reader = ReaderMock::new();

        let opts = Opts{
            subcmd: Some(SubCommand::Now),
            no_newlines: false,
            tag: None
        };

        handle(
            opts,
            &mut board,
            &mut writer,
            &mut editor,
            &reader
        );

        let output = writer.get_ref();
        assert_eq!(output, b"");
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
