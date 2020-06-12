use crate::web::Web;

// zip up config folder
// send to web service
// report back with success/failure
pub fn backup<W: Web>(web: &mut W) {
    let _ = web.send_backup();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::WebMock;

    #[test]
    fn it_sends_to_web_service() {
        let mut web = WebMock::new();

        backup(&mut web);

        assert!(web.send_backup_called());
    }

}
