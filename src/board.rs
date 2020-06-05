use crate::store::Store;
use crate::opts::{Task, Column};

pub trait BoardAccess {
    fn get_all_tasks(&self) -> Vec<Task>;
    fn get_column(&self, column: &str) -> Vec<Task>;
    fn create_task(&mut self, key: &str);
    fn update(&mut self, key: &str, task: Task);
    fn get(&self, key: &str) -> Option<Task>;
    fn remove(&mut self, key: &str);
    fn reindex_columns(&mut self) -> Result<usize, ()>;
    fn top_priority(&mut self, key: &str);
}

pub struct Board<'a, S: Store<Task>, C: Store<Vec<String>>> {
    store: &'a mut S,
    column_store: &'a mut C
}

impl <
    'a, S: Store<Task>, C: Store<Vec<String>>
> Board<'a, S, C> {
    pub fn new(
        store: &'a mut S,
        column_store: &'a mut C
    ) -> Board<'a, S, C> {
        Board{
            store,
            column_store
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

    fn find_in_column(
        &self,
        key: &str,
        column: &Vec<String>
    ) -> Option<usize> {
        column.iter().position(|x| x == key)
    }

    fn get_new_task(&self, key: &str) -> Task {
        Task{
            name: key.to_owned(),
            column: Column::Todo,
            description: None,
            tags: None
        }
    }
}

impl <
    'a, S: Store<Task>, C: Store<Vec<String>>
> BoardAccess for Board<'a, S, C> {
    fn get_all_tasks(&self) -> Vec<Task> {
        self.store.get_all()
    }

    fn get_column(&self, column: &str) -> Vec<Task> {
        let list = self.get_column_list(column);
        list.iter().filter_map(|key| {
            self.store.get(&key)
        }).collect()
    }

    fn create_task(&mut self, key: &str){
        let task = self.get_new_task(key);
        self.store.set(key, task);
        self.add_to_column(key, Column::Todo);
    }

    fn get(&self, key: &str) -> Option<Task>{
        self.store.get(key)
    }

    fn update(&mut self, key: &str, task: Task) {
        self.remove(key);
        self.store.set(key, task.clone());
        self.add_to_column(key, task.column);
    }

    fn remove(&mut self, key: &str){
        match self.store.get(key) {
            Some(task) => {
                let col_label = self.get_column_label(task.column);
                let mut col = self.get_column_list(&col_label);
                let index = self.find_in_column(key, &col)
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
                let index = self.find_in_column(key, &col)
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
        let board = Board::new(&mut store, &mut col_store);

        let tasks = board.get_all_tasks();

        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn it_can_create_a_task_with_a_key() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("test");
        let task = get_task("test", Column::Todo);

        assert!(store.set_called_with("test", &task));
    }

    #[test]
    fn it_adds_new_tasks_to_todo() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("test");

        assert_eq!(
            col_store.get("todo").unwrap(),
            vec!("test".to_owned())
        );
    }

    #[test]
    fn new_tasks_added_to_bottom_of_todo_column() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("test1");
        board.create_task("test2");

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
        let board = Board::new(&mut store, &mut col_store);

        let returned_task = board.get("test");

        assert_eq!(returned_task.unwrap(), task.clone());
    }

    #[test]
    fn it_can_rm_a_task_with_a_key() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);
        board.create_task("test");
        board.remove("test");

        assert!(store.rm_called_with("test"));
    }

    #[test]
    fn removing_a_task_removes_it_from_the_column() {
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("test");
        board.remove("test");

        assert_eq!(col_store.get("todo").unwrap().len(), 0);
    }

    #[test]
    fn it_can_update() {
        let task = get_task("test", Column::Todo);

        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.update("test", task.clone());

        assert!(store.set_called_with("test", &task));
    }

    #[test]
    fn update_moves_items_between_columns() {
        let task = get_task("test", Column::Doing);

        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.create_task("test");
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

        let board = Board::new(&mut store, &mut col_store);
        let tasks = board.get_column("doing");
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn it_can_reindex_columns() {
        let task = get_task("test", Column::Doing);

        let mut store = StoreMock::new();
        store.set("test", task.clone());
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.reindex_columns();

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

        let mut board = Board::new(&mut store, &mut col_store);
        board.top_priority("task1");
        let col = board.get_column("doing");
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

        let mut board = Board::new(&mut store, &mut col_store);
        board.top_priority("task1");
        board.update("task1", get_task("task1", Column::Done));
        assert_eq!(board.get_column("done").len(), 1);
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

        let mut board = Board::new(&mut store, &mut col_store);
        board.top_priority("task1");
        board.top_priority("task2");
        assert_eq!(
            board.get_column("doing").get(0).unwrap(),
            &get_task("task2", Column::Doing)
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
