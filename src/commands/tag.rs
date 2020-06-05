use std::io::Write;
use crate::board::BoardAccess;
use crate::opts::Task;

pub fn tag<B: BoardAccess, W: Write>(
    key: &str,
    tag_label: Option<String>,
    board: &mut B,
    writer: &mut W
) {
    let task_result = board.get(key);

    if task_result.is_none() {
        let _ = write!(
            writer, "No task called '{}' found.\n", key
        );
        return;
    }

    let task = task_result.expect("unable to unwrap task");

    match tag_label {
        None => view_tags(task, writer),
        Some(t) => add_tag(task, t, board)
    }
}

fn view_tags<W: Write>(task: Task, writer: &mut W) {
    let tags = match task.tags {
        Some(t) => {
            t.join(", ")
        },
        None => "[No tags]".to_owned()
    };
    let _ = write!(writer, "{}\n", tags);
}

fn add_tag<B: BoardAccess>(
    task: Task,
    tag: String,
    board: &mut B
) {
    let mut new_task = task.clone();
    let mut tag_list = new_task.tags.unwrap_or(vec!());

    tag_list.push(tag.clone()); 
    new_task.tags = Some(tag_list);

    board.update(&task.name, new_task);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;
    use crate::test::StoreMock;
    use crate::board::Board;
    use crate::opts::Column;
    use std::{str, io::Cursor};

    #[test]
    fn it_outputs_the_tasks_tags_when_no_new_tag_is_set() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut task = get_task("task", Column::Todo);
        task.tags = Some(vec!("tag1".to_owned()));

        store.set("task", task);
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

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
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

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
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        tag("task", None, &mut board, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"tag1, tag2\n");
    }

    #[test]
    fn it_exits_gracefully_when_no_task_is_found() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );


        tag("task", None, &mut board, &mut writer);

        let output = writer.get_ref();
        assert_eq!(output, b"No task called 'task' found.\n");

    }

    #[test]
    fn it_adds_a_tag_when_one_is_passed() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("task");
        let tag_label = Some("tag".to_string());

        tag("task", tag_label, &mut board, &mut writer);

        let changed_task = store.get("task").unwrap();

        assert_eq!(
            changed_task.tags,
            Some(vec!("tag".to_string()))
        );
    }

    #[test]
    fn it_indexes_tags_when_they_are_created() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("task");
        let tag_label = Some("tag".to_string());

        tag("task", tag_label, &mut board, &mut writer);

        let tag_index = tag_store.get("tag").unwrap();

        assert_eq!(
            tag_index,
            vec!("task".to_string())
        );
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

