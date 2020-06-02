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


// the board needs to be able to
// get all tasks ( <-- replaces existing functionality)
// get by key    ( <-- replaces existing functionality)
// set new task  ( <-- replaces existing functionality)
// update        ( <-- replaces existing functionality)
// rm            ( <-- replaces existing functionality)
// get via column
// move between columns

// reorder tasks
pub struct Board<'a, S: Store<Task>> {
    store: &'a mut S
}

impl <'a, S: Store<Task>> Board<'a, S> {
    pub fn new(store: &'a mut S) -> Board<'a, S> {
        Board{
            store
        }
    }
}

impl <'a, S: Store<Task>> BoardAccess for Board<'a, S> {
    fn get_all_tasks(&self) -> Vec<Task> {
        self.store.get_all()
    }

    fn get_column(&self, _column: &str) -> Vec<Task> {
        unimplemented!()
/*
        let list_result = self.store.get(
            column, COLUMN_BUCKET_KEY
        );

        match list_result {
            Some(StoreValue::List(x)) => {
                x.iter().map(|key| {
                    self.store.get(&key, TASK_BUCKET_KEY)
                        .unwrap().task().unwrap()
                }).collect::<Vec<Task>>()
            },
            _    => vec!()
        }
*/
    }

    fn create_task(&mut self, key: &str){
        let task = Task{
            name: key.to_string(),
            column: Column::Todo,
            description: None
        };
        self.store.set(
            key,
            task
        );
    }

    fn get(&self, key: &str) -> Option<Task>{
        self.store.get(key)
    }

    fn update(&mut self, key: &str, task: Task) {
        self.store.set(key, task)
    }

    fn remove(&mut self, key: &str){
        self.store.rm(key)
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

        let board = Board::new(&mut store);

        let tasks = board.get_all_tasks();

        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn it_can_create_a_task_with_a_key() {
        let mut store = StoreMock::new();
        let mut board = Board::new(&mut store);

        board.create_task("test");
        let task = Task {
            name: "test".to_string(),
            column: Column::Todo,
            description: None
        };

        assert!(store.set_called_with("test", &task));
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
        let board = Board::new(&mut store);

        let returned_task = board.get("test");

        assert_eq!(returned_task.unwrap(), task.clone());
    }

    #[test]
    fn it_can_rm_a_task_with_a_key() {
        let mut store = StoreMock::new();
        let mut board = Board::new(&mut store);

        board.remove("test");

        assert!(store.rm_called_with("test"));
    }

    #[test]
    fn it_can_get_update() {
        let task = Task {
            name: "test".to_string(),
            column: Column::Todo,
            description: None
        };

        let mut store = StoreMock::new();
        let mut board = Board::new(&mut store);

        board.update("test", task.clone());

        assert!(store.set_called_with("test", &task));
    }

/*
    #[test]
    fn it_can_return_tasks_by_column() {
        let mut store = StoreMock::new();

        store.insert_tasks(vec!(
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
        ));

        store.bulk_insert(vec!(
            ("doing", vec!(
                    "task2".to_owned(),
                    "task1".to_owned()
                )
            ),
        ));

        let board = Board::new(&mut store);

        let tasks = board.get_column("doing");

        assert_eq!(tasks.len(), 2);
    }
*/

}
