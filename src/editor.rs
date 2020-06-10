use std::{
    path::PathBuf,
    fs, fs::File,
    process::Command,
};


pub trait Editor {
    fn open(&mut self, path: &str);
    fn create(&mut self, key: &str) -> Result<String, ()>;
}

pub struct FileEditor {
    default_editor: Option<String>,
    root_path: String
}

impl FileEditor {
    pub fn new(default_editor: Option<String>, root_path: String) -> FileEditor {
        FileEditor{
            default_editor,
            root_path
        }
    }

    fn open_editor(&self, path: &str) {
        if self.default_editor.as_ref().is_none() {
            // error
            return
        }

        let editor = self.default_editor.as_ref().unwrap();

        Command::new(&editor)
            .arg(path)
            .status()
            .expect("Something went wrong");
    }
}

impl Editor for FileEditor {
    fn open(&mut self, path: &str) {
        self.open_editor(path);
    }

    fn create(&mut self, key: &str) -> Result<String, ()> {
        let mut path = PathBuf::new();
        path.push(&self.root_path);
        path.push(key);
        fs::create_dir_all(&self.root_path)
            .expect("unable to create files dir");
        File::create(&path).expect("Could not create file");
        let output_path = path.to_str()
            .expect("unable to get path");
        self.open_editor(output_path);
        Ok(output_path.to_string())
    }
}

#[cfg(test)]
mod tests {
//    use super::*;

}
