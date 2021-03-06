use crate::web::Web;
use std::fs::File;

pub struct WebMock{
    is_send_to_backup_called: bool
}

impl WebMock {
    pub fn new() -> WebMock {
        WebMock{
            is_send_to_backup_called: false
        }
    }

    pub fn send_backup_called(&self) -> bool {
        self.is_send_to_backup_called
    }
}

impl Web for WebMock {
    fn send_backup(&mut self, _: &str, _: File) -> Result<(), ()> {
        self.is_send_to_backup_called = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_true_when_send_to_backup_is_called() {
        let file = File::create("foo.txt").unwrap();
        let mut web = WebMock::new();
        let _ = web.send_backup("abc", file);

        assert!(web.send_backup_called());
    }

    #[test]
    fn it_returns_false_when_send_to_backup_is_not_called() {
        let web = WebMock::new();
        assert!(!web.send_backup_called());
    }
}
