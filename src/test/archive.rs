use std::fs::File;
use crate::archive::Archive;

pub struct ArchiveMock;

impl ArchiveMock {
    pub fn new() -> ArchiveMock {
        ArchiveMock
    }
}

impl Archive for ArchiveMock {
    fn write(&self) -> File {
        File::create("/tmp/archivemocktest").unwrap()
    }
}

