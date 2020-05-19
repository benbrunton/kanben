use crate::opts::*;
use crate::store::Store;

pub struct StoreMock {
    last_set_key: Option<String>,
    rm_calls: Vec<String>,
    last_set_task: Option<Task>,
    tasks: Vec<Task>
}

impl StoreMock {
    pub fn new() -> StoreMock {
        StoreMock{
            last_set_key: None,
            last_set_task: None,
            rm_calls: vec!(),
            tasks: vec!()
        }
    }

    pub fn set_called_with(&self, key: &str, task: &Task) -> bool {
        if self.last_set_task.is_none()
            || self.last_set_key.is_none() {
            return false;
        }

        let task_matches = task.clone() == self.last_set_task
            .as_ref().unwrap().clone();
        let key_matches = key == self.last_set_key
            .as_ref().unwrap();

        task_matches && key_matches
    }

    pub fn set_called(&self) -> bool {
        self.last_set_key.is_some()
    }

    pub fn rm_called_with(&self, key: &str) -> bool {
        self.rm_calls.iter().any(|k| k == key)
    }

    pub fn insert_tasks(&mut self, tasks: Vec<(&str, Task)>) {
        self.tasks = tasks.iter()
            .map(|(_, task)| task.clone()).collect();
    }
}

impl Store for StoreMock {
    fn get_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    fn set(&mut self, key: &str, value: Task) {
        self.last_set_key = Some(String::from(key));
        self.last_set_task = Some(value); 
    }

    fn get(&self, _key: &str) -> Option<Task> {
        unimplemented!()
    }

    fn rm(&mut self, key: &str) {
        self.rm_calls.push(String::from(key));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_records_what_is_set() {
        let mut store = StoreMock::new();
        let name = String::from("test");
        let task = Task {
            name: name.clone(),
            column: Column::Doing,
            description: String::from("") 
        };

        store.set(&name, task.clone());

        assert!(store.set_called_with(&name, &task));
    }

    #[test]
    fn it_returns_false_if_set_not_called_with_passed_values() {
        let mut store = StoreMock::new();
        let name = String::from("test");
        let passed_task = Task {
            name: String::from("fake test"),
            column: Column::Doing,
            description: String::from("") 
        };

        let checked_task = Task {
            name: name.clone(),
            column: Column::Doing,
            description: String::from("") 
        };


        store.set(&name, passed_task.clone());

        assert!(!store.set_called_with(&name, &checked_task));
    }


    #[test]
    fn it_reports_on_whether_set_is_called() {
        let store = StoreMock::new();
        
        assert!(!store.set_called());
    }

    #[test]
    fn it_returns_true_when_set_was_called() {
        let mut store = StoreMock::new();
        let name = String::from("test");
        let task = Task {
            name: String::from("fake test"),
            column: Column::Doing,
            description: String::from("") 
        };

        store.set(&name, task.clone());

        assert!(store.set_called());
    }

    #[test]
    fn it_can_bulk_add_tasks() {
        let mut store = StoreMock::new();
        let name = String::from("test");
        let task = Task {
            name: String::from("fake test"),
            column: Column::Doing,
            description: String::from("") 
        };

        store.insert_tasks(vec!((&name, task.clone())));

        assert_eq!(store.get_all(), vec!(task.clone()));
    }

    #[test]
    fn it_can_report_on_calls_to_rm() {
        let mut store = StoreMock::new();
        store.rm("test");
        assert!(store.rm_called_with("test"));
    }

    #[test]
    fn it_can_report_multiple_calls_to_rm() {
        let mut store = StoreMock::new();
        store.rm("test");
        store.rm("test2");
        assert!(store.rm_called_with("test"));
        assert!(store.rm_called_with("test2"));
    }
}
