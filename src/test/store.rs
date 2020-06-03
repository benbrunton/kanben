use crate::opts::*;
use crate::store::Store;
use std::collections::HashMap;

pub struct StoreMock<T> {
    is_set_called: bool,
    rm_calls: Vec<String>,
    temp_store: HashMap<String, T>,
}

impl <T: std::cmp::PartialEq + std::clone::Clone> StoreMock<T> {
    pub fn new() -> StoreMock<T>{
        StoreMock{
            is_set_called: false,
            rm_calls: vec!(),
            temp_store: HashMap::new(),
        }
    }

    pub fn set_called_with(&self, key: &str, task: &T) -> bool {
        let value = self.temp_store.get(key);
        match value {
            Some(v) => v == task,
            None => false
        }
    }

    pub fn set_called(&self) -> bool {
        self.is_set_called
    }

    pub fn rm_called_with(&self, key: &str) -> bool {
        self.rm_calls.iter().any(|k| k == key)
    }

    pub fn bulk_insert(
        &mut self,
        items: Vec<(&str, T)>
    ) {
        for (key, item) in items.iter() {
            self.temp_store.insert(key.to_string(), item.to_owned());
        }
    }
}

impl <T: std::clone::Clone> Store<T> for StoreMock <T> {
    fn get_all(&self) -> Vec<T> {
        self.temp_store.values()
            .map(|item| item.to_owned())
            .collect::<Vec<T>>()
    }

    fn set(&mut self, key: &str, value: T) {
        self.is_set_called = true;
        self.temp_store.insert(key.to_string(), value);
    }

    fn get(&self, key: &str) -> Option<T> {
        match self.temp_store.get(key) {
            Some(v) => Some(v.clone()),
            _       => None
        }
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
            description: None
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
            description: None
        };

        let checked_task = Task {
            name: name.clone(),
            column: Column::Doing,
            description: None
        };


        store.set(&name, passed_task.clone());
        assert!(!store.set_called_with(&name, &checked_task));
    }


    #[test]
    fn it_reports_on_whether_set_is_called() {
        let store: StoreMock<()> = StoreMock::new();
        
        assert!(!store.set_called());
    }

    #[test]
    fn it_returns_true_when_set_was_called() {
        let mut store = StoreMock::new();
        let name = String::from("test");
        let task = Task {
            name: String::from("fake test"),
            column: Column::Doing,
            description: None
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
            description: None
        };

        store.bulk_insert(vec!((&name, task.clone())));

        assert_eq!(
            store.get_all(),
            vec!(task.clone())
        );
    }

    #[test]
    fn it_can_report_on_calls_to_rm() {
        let mut store: StoreMock<()> = StoreMock::new();
        store.rm("test");
        assert!(store.rm_called_with("test"));
    }

    #[test]
    fn it_can_report_multiple_calls_to_rm() {
        let mut store: StoreMock<()> = StoreMock::new();
        store.rm("test");
        store.rm("test2");
        assert!(store.rm_called_with("test"));
        assert!(store.rm_called_with("test2"));
    }

    #[test]
    fn it_can_set_the_response_of_get() {
        let mut store = StoreMock::new();
        let task = Task {
            name: String::from("test"),
            column: Column::Doing,
            description: None
        };

        store.set("test", task.clone());

        let returned_task: Task = store.get("test").unwrap();

        assert_eq!(
            returned_task,
            task.clone()
        );
    }

    #[test]
    fn it_can_bulk_add_for_get_by_key() {
        let mut store = StoreMock::new();
        let key = String::from("test");
        let task = Task {
            name: String::from("fake test"),
            column: Column::Doing,
            description: None
        };

        store.bulk_insert(vec!((&key, task.clone())));

        assert_eq!(
            store.get(&key),
            Some(task.clone())
        );
    }

}
