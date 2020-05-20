use crate::editor::Editor;

pub struct EditorMock{
    last_open_call: Option<String>,
    last_create_call: Option<String>,
    create_response: Result<String, ()>,
}

impl EditorMock {
    pub fn new() -> EditorMock {
        EditorMock{
            last_open_call: None,
            last_create_call: None,
            create_response: Ok("filepath".to_string())
        }
    }

    pub fn open_called_with(&self, key_path: &str) -> bool {
        match &self.last_open_call {
            Some(a) => a == key_path,
            _  => false
        }
    }

    pub fn open_called(&self) -> bool {
        self.last_open_call.is_some()
    }

    pub fn create_called_with(&self, key: &str) -> bool {
        match &self.last_create_call {
            Some(a) => a == key,
            _  => false
        }
    }

    pub fn return_from_create(&mut self, path: Result<String, ()>) {
        self.create_response = path.clone();
    }
}

impl Editor for EditorMock {
    fn open(&mut self, path: &str) {
        self.last_open_call = Some(path.to_string());
    }

    fn create(&mut self, key: &str) -> Result<String, ()> {
        self.last_create_call = Some(key.to_string());
        self.create_response.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_match_on_open_parameters() {
        let mut editor = EditorMock::new();

        editor.open("abc");

        assert!(editor.open_called_with("abc"));
    }

    #[test]
    fn it_can_report_when_open_is_called() {
        let mut editor = EditorMock::new();

        editor.open("abc");

        assert!(editor.open_called());
    }

    #[test]
    fn it_returns_false_when_open_is_not_called() {
        let editor = EditorMock::new();
        assert!(!editor.open_called());
    }

    #[test]
    fn it_reports_when_create_has_been_called() {
        let mut editor = EditorMock::new();
        let _ = editor.create("abc");
        assert!(editor.create_called_with("abc"));
    }
}
