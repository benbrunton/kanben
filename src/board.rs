use crate::store::Store;
use crate::opts::{Task, Column};

pub trait BoardAccess {
    fn get_all_tasks(&self) -> Vec<Task>;
    fn get_column(&self, column: &str) -> Vec<Task>;
    fn create_task(&mut self, key: &str);
    fn update(&mut self, key: &str, task: Task);
    fn get(&self, key: &str) -> Option<Task>;
    fn remove(&mut self, key: &str);
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

    fn get_new_task(&self, key: &str) -> Task {
        Task{
            name: key.to_owned(),
            column: Column::Todo,
            description: None
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
        list.iter().map(|key| self.store.get(&key).unwrap())
            .collect()
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
                let index = col.binary_search(&key.to_owned())
                    .unwrap();
                col.remove(index);
                self.column_store.set(&col_label, col);
                self.store.rm(key)
            },
            None => ()
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
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None,
            }),
            ("task2", Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None,
            }),
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
        let task = Task {
            name: "test".to_string(),
            column: Column::Todo,
            description: None
        };

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
        let task = Task {
            name: "test".to_string(),
            column: Column::Todo,
            description: None
        };

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
        let task = Task {
            name: "test".to_string(),
            column: Column::Todo,
            description: None
        };

        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        board.update("test", task.clone());

        assert!(store.set_called_with("test", &task));
    }

    #[test]
    fn update_moves_items_between_columns() {
        let task = Task {
            name: "test".to_string(),
            column: Column::Doing,
            description: None
        };

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
            ("task1", Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None,
            }),
            ("task2", Task{
                name: String::from("task2"),
                column: Column::Doing,
                description: None,
            }),
            ("task4", Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None,
            })
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

}
