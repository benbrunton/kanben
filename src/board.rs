use crate::store::Store;
use crate::opts::{Task, Column};

pub trait BoardAccess {
    fn get_all_tasks(&self) -> Vec<Task>;
    fn get_column(&self, col: &str, tag: Option<String>) -> Vec<Task>;
    fn create_task(&mut self, key: &str, tag: Option<String>);
    fn update(&mut self, key: &str, task: Task);
    fn get(&self, key: &str) -> Option<Task>;
    fn remove(&mut self, key: &str);
    fn reindex_columns(&mut self) -> Result<usize, ()>;
    fn top_priority(&mut self, key: &str);
}

pub struct Board<'a, S: Store<Task>, C: Store<Vec<String>>> {
    store: &'a mut S,
    column_store: &'a mut C,
    tag_store: &'a mut C,
}

impl <
    'a, S: Store<Task>, C: Store<Vec<String>>
> Board<'a, S, C> {
    pub fn new(
        store: &'a mut S,
        column_store: &'a mut C,
        tag_store: &'a mut C
    ) -> Board<'a, S, C> {
        Board{
            store,
            column_store,
            tag_store
        }
    }

    fn add_to_column(&mut self, key: &str, column: Column) {
        let label = self.get_column_label(column);
        let mut list = self.get_column_list(&label);
        list.push(key.to_owned());
        self.column_store.set(&label, list);
    }

    fn get_column_list(&self, label: &str) -> Vec<String> {
        match self.column_store.get(label) {
            Some(x) => x,
            _       => vec!()
        }
    }

    fn get_column_label(&self, column: Column) -> String {
        let mut label = format!("{:?}", column);
        label.make_ascii_lowercase();
        label
    }

    fn find_in_list(
        &self,
        key: &str,
        list: &Vec<String>
    ) -> Option<usize> {
        list.iter().position(|x| x == key)
    }

    fn get_new_task(&self, key: &str, tag: Option<String>) -> Task {
        let tags = match tag {
            None => None,
            Some(t) => Some(vec!(t.clone()))
        };

        Task{
            name: key.to_owned(),
            column: Column::Todo,
            description: None,
            tags
        }
    }

    fn get_tag_diff(
        &self,
        old_tags: Option<Vec<String>>,
        new_tags: Option<Vec<String>>
    ) -> (Vec<String>, Vec<String>) {
        let mut add_tags = vec!();
        let mut rm_tags = vec!();
        let new_tags = new_tags.unwrap_or(vec!());
        if old_tags.is_none() {
            add_tags = new_tags;
        } else {
            let old_tags = old_tags.unwrap(); 
            // todo extract
            for tag in old_tags.iter() {
                if !new_tags.iter().any(|i| i == tag ){
                    rm_tags.push(tag.clone());
                }
            }

            for tag in new_tags.iter() {
                if !old_tags.iter().any(|i| i == tag) {
                    add_tags.push(tag.clone());
                }
            }
        }

        (add_tags, rm_tags)
    }

    fn index_tags(&mut self, tags: Vec<String>, key: &str) {
        for tag in tags {
            let mut tag_list = self.tag_store.get(&tag)
                .unwrap_or(vec!());
            tag_list.push(key.to_owned());
            tag_list.dedup();
            self.tag_store.set(&tag, tag_list);
        }
    }

    fn rm_tag_index(&mut self, tags: Vec<String>, key: &str) {
        for tag in tags {
            let mut tag_list = self.tag_store.get(&tag)
                .unwrap_or(vec!());
            let index = self.find_in_list(key, &tag_list)
                .expect(
                    &format!(
                        "can't find '{}' in list '{:?}'",
                        tag, tag_list
                    )
                );
            tag_list.remove(index);
            self.tag_store.set(&tag, tag_list);
        }
    }

}

impl <
    'a, S: Store<Task>, C: Store<Vec<String>>
> BoardAccess for Board<'a, S, C> {
    fn get_all_tasks(&self) -> Vec<Task> {
        self.store.get_all()
    }

    fn get_column(
        &self,
        column: &str,
        tag: Option<String>
    ) -> Vec<Task> {
        let list = self.get_column_list(column);
        let tag_result = match &tag {
            Some(t) => self.tag_store.get(&t),
            None => None
        };

        list.iter().filter(|&key|{
            match &tag_result {
                Some(v) => v.iter().any(|x| x == key),
                None => tag.is_none()
            }
        }).filter_map(|key| {
            self.store.get(&key)
        }).collect()
    }

    fn create_task(&mut self, key: &str, tag: Option<String>){
        let task = self.get_new_task(key, tag.clone());
        self.store.set(key, task);
        self.add_to_column(key, Column::Todo);

        if tag.is_some() {
            let tags = vec!(tag.unwrap().clone());
            self.index_tags(tags, key);
        }
    }

    fn get(&self, key: &str) -> Option<Task>{
        self.store.get(key)
    }

    fn update(&mut self, key: &str, task: Task) {
        let old_task_result = self.get(key); 

        if old_task_result.is_none() {
            return;
        }

        let old_task = old_task_result.unwrap();
        let old_tags = old_task.tags;

        let (add_tags, rm_tags) = self.get_tag_diff(
            old_tags,
            task.tags.clone()
        );

        self.index_tags(add_tags, key);
        self.rm_tag_index(rm_tags, key);

        if old_task.column != task.column.clone() {
            self.remove(key);
            self.add_to_column(key, task.column.clone());
        }

        self.store.set(key, task.clone());
    }

    fn remove(&mut self, key: &str){
        match self.store.get(key) {
            Some(task) => {
                let col_label = self.get_column_label(task.column);
                let mut col = self.get_column_list(&col_label);
                let index = self.find_in_list(key, &col)
                    .expect(
                        &format!(
                            "can't find '{}' in column '{}'",
                            key, col_label
                        )
                    );
                col.remove(index);
                self.column_store.set(&col_label, col);
                self.store.rm(key)
            },
            None => ()
        }
    }

    fn reindex_columns(&mut self) -> Result<usize, ()>{
        self.column_store.set("todo", vec!());
        self.column_store.set("doing", vec!());
        self.column_store.set("done", vec!());
        let tasks = self.store.get_all();
        for task in tasks.iter() {
            self.add_to_column(&task.name, task.column.clone());
        }
        Ok(tasks.len())
    }

    fn top_priority(&mut self, key: &str) {
        let task_result = self.store.get(key);
        match task_result {
            Some(task) => {
                let label = self.get_column_label(task.column);
                let mut col = self.get_column_list(&label);
                let index = self.find_in_list(key, &col)
                    .expect(
                        &format!(
                            "can't find '{}' in column '{}'",
                            key, label
                        )
                    );
                col.remove(index);
                col.insert(0, task.name.clone());
                self.column_store.set(&label, col);
            },
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opts::Column;
    use crate::test::StoreMock;

    #[test]
    fn it_can_get_all_tasks() {
        let mut store = StoreMock::new();

        store.bulk_insert(vec!(
            ("task1", get_task("task1", Column::Doing)),
            ("task2", get_task("task2", Column::Todo)),
        ));
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        let tasks = board.get_all_tasks();

        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn it_can_create_a_task_with_a_key() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("test", None);
        let task = get_task("test", Column::Todo);

        assert!(store.set_called_with("test", &task));
    }

    #[test]
    fn it_adds_new_tasks_to_todo() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );


        board.create_task("test", None);

        assert_eq!(
            col_store.get("todo").unwrap(),
            vec!("test".to_owned())
        );
    }

    #[test]
    fn new_tasks_added_to_bottom_of_todo_column() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("test1", None);
        board.create_task("test2", None);

        assert_eq!(
            col_store.get("todo").unwrap(),
            vec!("test1".to_owned(), "test2".to_owned())
        );

    }

    #[test]
    fn it_can_get_a_task_with_a_key() {
        let task = get_task("test", Column::Todo);

        let mut store = StoreMock::new();
        store.set("test", task.clone());
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        let returned_task = board.get("test");

        assert_eq!(returned_task.unwrap(), task.clone());
    }

    #[test]
    fn it_can_rm_a_task_with_a_key() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("test", None);
        board.remove("test");

        assert!(store.rm_called_with("test"));
    }

    #[test]
    fn removing_a_task_removes_it_from_the_column() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("test", None);
        board.remove("test");

        assert_eq!(col_store.get("todo").unwrap().len(), 0);
    }

    #[test]
    fn it_can_update() {
        let task = get_task("test", Column::Todo);

        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("test", None);
        board.update("test", task.clone());

        assert!(store.set_called_with("test", &task));
    }

    #[test]
    fn update_moves_items_between_columns() {
        let task = get_task("test", Column::Doing);

        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("test", None);
        board.update("test", task.clone());

        assert_eq!(col_store.get("doing").unwrap().len(), 1);
    }

    #[test]
    fn it_can_return_tasks_by_column() {
        let mut store = StoreMock::new();

        store.bulk_insert(vec!(
            ("task1", get_task("task1", Column::Doing)),
            ("task2", get_task("task2", Column::Doing)),
            ("task4", get_task("task4", Column::Todo)),
        ));

        let mut col_store = StoreMock::new();
        col_store.bulk_insert(vec!(
            ("doing", vec!(
                    "task2".to_owned(),
                    "task1".to_owned()
                )
            ),
        ));

        let mut tag_store = StoreMock::new();
        let board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        let tasks = board.get_column("doing", None);
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn it_can_reindex_columns() {
        let task = get_task("test", Column::Doing);

        let mut store = StoreMock::new();
        store.set("test", task.clone());
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        let _ = board.reindex_columns();

        assert_eq!(col_store.get("doing").unwrap().len(), 1);

    }

    #[test]
    fn it_can_make_an_item_top_priority() {
        let mut store = StoreMock::new();

        store.bulk_insert(vec!(
            ("task1", get_task("task1", Column::Doing)),
            ("task2", get_task("task2", Column::Doing)),
            ("task4", get_task("task2", Column::Todo)),
        ));

        let mut col_store = StoreMock::new();
        col_store.bulk_insert(vec!(
            ("doing", vec!(
                    "task2".to_owned(),
                    "task1".to_owned()
                )
            ),
        ));

        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.top_priority("task1");
        let col = board.get_column("doing", None);
        let task1 = col.get(0).unwrap();
        assert_eq!(&task1.name, "task1");
    }

    #[test]
    fn it_can_move_an_item_after_making_it_priority() {
        let mut store = StoreMock::new();

        store.bulk_insert(vec!(
            ("task1", get_task("task1", Column::Doing)),
            ("task2", get_task("task2", Column::Doing)),
            ("task4", get_task("task4", Column::Todo))
        ));

        let mut col_store = StoreMock::new();
        col_store.bulk_insert(vec!(
            ("doing", vec!(
                    "task2".to_owned(),
                    "task1".to_owned()
                )
            ),
        ));

        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.top_priority("task1");
        board.update("task1", get_task("task1", Column::Done));
        assert_eq!(board.get_column("done", None).len(), 1);
    }

    #[test]
    fn it_can_repeat_a_priority_action() {
        let mut store = StoreMock::new();

        store.bulk_insert(vec!(
            ("task1", get_task("task1", Column::Doing)),
            ("task2", get_task("task2", Column::Doing)),
            ("task4", get_task("task4", Column::Todo))
        ));

        let mut col_store = StoreMock::new();
        col_store.bulk_insert(vec!(
            ("doing", vec!(
                    "task2".to_owned(),
                    "task1".to_owned()
                )
            ),
        ));
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.top_priority("task1");
        board.top_priority("task2");
        assert_eq!(
            board.get_column("doing", None).get(0).unwrap(),
            &get_task("task2", Column::Doing)
        );
    }

    #[test]
    fn it_indexes_tags_when_they_are_created() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("task", None);
        let mut new_task = get_task("task", Column::Todo);
        new_task.tags = Some(vec!("tag".to_owned()));
        board.update("task", new_task);

        let tag_index = tag_store.get("tag").unwrap();

        assert_eq!(
            tag_index,
            vec!("task".to_string())
        );
    }

    #[test]
    fn it_adds_to_existing_indexes_when_there_is_a_tag() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        tag_store.set("tag", vec!("task2".to_owned()));
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("task", None);
        let mut new_task = get_task("task", Column::Todo);
        new_task.tags = Some(vec!("tag".to_owned()));
        board.update("task", new_task);

        let tag_index = tag_store.get("tag").unwrap();

        assert_eq!(
            tag_index,
            vec!("task2".to_owned(), "task".to_owned())
        );
    }

    #[test]
    fn it_doesnt_dupe_tag_indices() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        tag_store.set("tag", vec!("task".to_owned()));
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );
        board.create_task("task", None);
        let mut new_task = get_task("task", Column::Todo);
        new_task.tags = Some(vec!("tag".to_owned()));
        board.update("task", new_task);

        let tag_index = tag_store.get("tag").unwrap();

        assert_eq!(
            tag_index,
            vec!("task".to_owned())
        );
    }

    #[test]
    fn it_removes_old_tags() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        tag_store.set("tag", vec!("task".to_owned()));
        col_store.set("todo", vec!("task".to_owned()));
        let mut task = get_task("task", Column::Todo);
        task.tags = Some(vec!("tag".to_owned()));
        store.set("task", task);

        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        let new_task = get_task("task", Column::Todo);
        board.update("task", new_task);

        let tag_index = tag_store.get("tag").unwrap();
        let expected_list: Vec<String> = vec!();

        assert_eq!(
            tag_index,
            expected_list
        );
    }

    #[test]
    fn it_can_create_a_task_with_a_tag() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("test", Some("tag".to_owned()));
        let mut task = get_task("test", Column::Todo);
        task.tags = Some(vec!("tag".to_owned()));

        assert!(store.set_called_with("test", &task));
    }

    #[test]
    fn it_indexes_tags_from_new_tasks() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut tag_store = StoreMock::new();
        let mut board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        board.create_task("test", Some("tag".to_owned()));

        let tag_index = tag_store.get("tag").unwrap();

        assert_eq!(
            tag_index,
            vec!("test".to_string())
        );
    }

    #[test]
    fn it_can_return_tasks_by_column_filtered_by_tag() {
        let mut task1 = get_task("task1", Column::Doing);
        task1.tags = Some(vec!("tag".to_owned()));
        let mut task2 = get_task("task2", Column::Doing);
        task2.tags = Some(vec!("tag".to_owned()));
        let mut store = StoreMock::new();

        store.bulk_insert(vec!(
            ("task1", task1.clone()),
            ("task2", task2.clone()),
            ("task3", get_task("task3", Column::Doing)),
        ));

        let mut col_store = StoreMock::new();
        col_store.bulk_insert(vec!(
            ("doing", vec!(
                    "task1".to_owned(),
                    "task2".to_owned(),
                    "task3".to_owned(),
                )
            ),
        ));

        let mut tag_store = StoreMock::new();
        tag_store.set(
            "tag", vec!("task1".to_owned(), "task2".to_owned())
        );

        let board = Board::new(
            &mut store,
            &mut col_store,
            &mut tag_store
        );

        let tasks = board.get_column(
            "doing",
            Some("tag".to_owned())
        );
        assert_eq!(tasks, vec!(task1.clone(), task2.clone()));
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
