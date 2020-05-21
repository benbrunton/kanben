use crate::board::BoardAccess;
use crate::opts::{Task, Column};
use std::collections::HashMap;

pub struct BoardMock {
    tasks: Vec<Task>,
    set_tasks: HashMap<String, Task>,
    create_task: Option<String>,
    update_task: Option<(String, Task)>,
    remove_tasks: Vec<String>
}

impl BoardMock {

    pub fn new() -> BoardMock{
        BoardMock{
            tasks: vec!(),
            set_tasks: HashMap::new(),
            create_task: None,
            update_task: None,
            remove_tasks: vec!(),
         }
    }

    pub fn set_tasks(&mut self, tasks: Vec<Task>){
        self.tasks = tasks;
    }

    pub fn set(&mut self, key: &str, task: Task) {
        self.set_tasks.insert(key.to_string(), task);
    }

    pub fn create_task_called_with(&self, key: &str) -> bool {
        match &self.create_task {
            Some(k) => key == k,
            None => false
        }
    }

    pub fn update_called_with(&self, key:&str, task:&Task) -> bool{
        match &self.update_task {
            Some((k, v)) => key == k && v == task,
            None => false
        }
    }

    pub fn remove_called_with(&self, key:&str) -> bool{
        self.remove_tasks.iter().any(|k| k == key)
    }

}

impl BoardAccess for BoardMock {
    fn get_all_tasks(&self) -> Vec<Task>{
        self.tasks.clone()
    }

    fn get_column(&self, _: Column) -> Vec<Task>{
        unimplemented!()
    }

    fn create_task(&mut self, key: &str){
        self.create_task = Some(key.to_string());
    }

    fn get(&self, key: &str) -> Option<Task>{
        match self.set_tasks.get(key) {
            Some(t) => Some(t.clone()),
            None => None
        }
    }

    fn update(&mut self, key: &str, task: Task){
        self.update_task = Some((key.to_string(), task));
    }

    fn remove(&mut self, key: &str){
        self.remove_tasks.push(key.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_set_all_tasks_to_be_returned() {
        let mut board_mock = BoardMock::new();
        let tasks = vec!(
            Task{
                name: String::from("task1"),
                column: Column::Doing,
                description: None,
            },
            Task{
                name: String::from("task2"),
                column: Column::Todo,
                description: None,
            }
        );

        board_mock.set_tasks(tasks.clone());

        assert_eq!(board_mock.get_all_tasks(), tasks.clone());
    }

    #[test]
    fn it_can_set_for_get() {
        let mut board_mock = BoardMock::new();
        let task = Task{
            name: "task1".to_string(),
            column: Column::Doing,
            description: None
        };

        board_mock.set("task1", task.clone());

        assert_eq!(board_mock.get("task1").unwrap(), task.clone());
    }

    #[test]
    fn it_returns_none_by_default() {
        let board_mock = BoardMock::new();
        assert_eq!(board_mock.get("task1"), None);
    }

    #[test]
    fn it_can_report_on_created_tasks() {
        let mut board_mock = BoardMock::new();

        board_mock.create_task("task1");

        assert!(
            board_mock.create_task_called_with("task1")
        );
    }

    #[test]
    fn it_returns_false_when_created_check_doesnt_match() {
        let board_mock = BoardMock::new();
        assert!(
            !board_mock.create_task_called_with("task1")
        );
    }

    #[test]
    fn it_can_report_on_updated_tasks() {
        let mut board_mock = BoardMock::new();

        let task = Task{
            name: "task1".to_string(),
            column: Column::Doing,
            description: None
        };
        board_mock.update("task1", task.clone());

        assert!(
            board_mock.update_called_with("task1", &task)
        );
    }

    #[test]
    fn it_returns_false_when_update_check_doesnt_match() {
        let board_mock = BoardMock::new();
        let task = Task{
            name: "task1".to_string(),
            column: Column::Doing,
            description: None
        };

        assert!(
            !board_mock.update_called_with("task1", &task)
        );
    }

    #[test]
    fn it_can_report_on_removed_tasks() {
        let mut board_mock = BoardMock::new();
        board_mock.remove("task1");
        assert!(
            board_mock.remove_called_with("task1")
        );
    }

    #[test]
    fn it_returns_false_when_remove_check_doesnt_match() {
        let board_mock = BoardMock::new();
        assert!(
            !board_mock.remove_called_with("task1")
        );
    }

    #[test]
    fn it_returns_true_for_all_remove_checks() {
        let mut board_mock = BoardMock::new();
        board_mock.remove("task1");
        board_mock.remove("task2");
        board_mock.remove("task3");
        assert!(board_mock.remove_called_with("task1"));
        assert!(board_mock.remove_called_with("task2"));
        assert!(board_mock.remove_called_with("task3"));

    }
}
