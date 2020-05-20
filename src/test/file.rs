use crate::file::Reader;

pub struct ReaderMock {
    read_output: String,
    read_gate: Option<String>
}

impl ReaderMock {
    pub fn new() -> ReaderMock{
        ReaderMock{
            read_output: "".to_string(),
            read_gate: None
        }
    }

    pub fn return_from_read(&mut self, txt: &str) {
        self.read_output = txt.to_string();
    }

    pub fn return_from_read_when(&mut self, key: &str, val: &str) {
        self.read_output = val.to_string();
        self.read_gate = Some(key.to_string());
    }
}

impl Reader for ReaderMock {
    fn read(&self, path: &str) -> Option<String> {
        match &self.read_gate {
            None => Some(self.read_output.clone()),
            Some(a) => {
                if a == &path.to_string() {
                    Some(self.read_output.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_set_the_read_output() {
        let mut reader = ReaderMock::new();
        reader.return_from_read("abc");
        assert_eq!(
            reader.read("test/path"),
            Some("abc".to_string())
        );
    }

    #[test]
    fn it_can_map_read_and_returns() {
        let mut reader = ReaderMock::new();
        reader.return_from_read_when("test/path", "bbb");
        assert_eq!(
            reader.read("test/path"),
            Some("bbb".to_string())
        );
    }

    #[test]
    fn it_returns_none_if_read_with_wrong_path() {
        let mut reader = ReaderMock::new();
        reader.return_from_read_when("test/path", "bbb");
        assert_eq!(reader.read("wrong/path"), None);
    }
}
