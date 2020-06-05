use std::io::Write;
use crate::board::BoardAccess;

pub fn tag<B: BoardAccess, W: Write>(
    key: &str,
    tag: Option<String>,
    board: &mut B,
    writer: &mut W
) {
    let task = board.get(key).unwrap();
    let tags = match task.tags {
        Some(t) => {
            t.join(", ")
        },
        None => "[No tags]".to_owned()
    };
    let _ = write!(writer, "{}\n", tags);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;
    use crate::test::StoreMock;
    use crate::board::Board;
    use crate::opts::{Task, Column};
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_the_tasks_tags_when_no_new_tag_is_set() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut task = get_task("task", Column::Todo);
        task.tags = Some(vec!("tag1".to_owned()));

        store.set("task", task);
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        tag("task", None, &mut board, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"tag1\n");
    }

    #[test]
    fn it_outputs_a_message_when_there_are_no_tags() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let task = get_task("task", Column::Todo);

        store.set("task", task);
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        tag("task", None, &mut board, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"[No tags]\n");
    }

    #[test]
    fn it_outputs_multiple_tags_with_a_comma_delimit() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut task = get_task("task", Column::Todo);
        task.tags = Some(vec!(
            "tag1".to_owned(),
            "tag2".to_owned()
        ));

        store.set("task", task);
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        tag("task", None, &mut board, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"tag1, tag2\n");
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

