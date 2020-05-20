use std::{fs::File, io::Read};

pub trait Reader {
    fn read(&self, path: &str) -> Option<String>;
}

pub struct FileReader {

}

impl FileReader {
    pub fn new() -> FileReader {
        FileReader{}
    }
}

impl Reader for FileReader {
    fn read(&self, path: &str) -> Option<String> {
        let mut editable = String::new();
        File::open(path)
            .expect("Could not open file")
            .read_to_string(&mut editable)
            .expect("Could not read file");

        Some(editable)
    }
}
