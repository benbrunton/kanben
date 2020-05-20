use std::{fs::File, io::Read};

pub trait Reader {
    fn read(&self, path: &str) -> Option<String>;
}

pub struct FileReader;

impl FileReader {
    pub fn new() -> FileReader {
        FileReader
    }
}

impl Reader for FileReader {
    fn read(&self, path: &str) -> Option<String> {
        let mut editable = String::new();
        let file_open_result = File::open(path);
        if file_open_result.is_err() {
            return None;
        }

        let read_result = file_open_result.unwrap()
            .read_to_string(&mut editable);

        if read_result.is_err() {
            return None;

        }

        Some(editable)
    }
}
