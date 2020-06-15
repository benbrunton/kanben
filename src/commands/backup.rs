use crate::web::Web;
use crate::archive::Archive;
use log::info;

// zip up config folder
// send to web service
// report back with success/failure
pub fn backup<W: Web, A: Archive>(web: &mut W, archive: &A) {
    info!("backing up");
    let file = archive.write();
    info!("file written");
    let r = web.send_backup("test-backup-1", file);

    info!("backup successful: {}", r.is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::WebMock;
    use crate::test::ArchiveMock;

    #[test]
    fn it_sends_to_web_service() {
        let mut web = WebMock::new();
        let mut archive = ArchiveMock::new();

        backup(&mut web, &archive);

        assert!(web.send_backup_called());
    }

}
