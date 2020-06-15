use std::fs::{File, remove_dir_all};
use std::time::SystemTime;
use std::path::Path;
use std::io::{Read, Write};
use zip::ZipWriter;
use fs_extra::dir::{copy, CopyOptions};
use log::info;
use walkdir::WalkDir;

pub trait Archive {
    fn write(&self) -> File;
}

pub struct ZipArchive {
    path: String
}

impl ZipArchive {
    pub fn new(path: &str) -> ZipArchive {
        ZipArchive{ path: path.to_owned() }
    }

    fn get_new_file(&self, filepath: &str) -> File {
        info!("create file: [{}]", &filepath);
        File::create(&filepath).unwrap()
    }
}

impl Archive for ZipArchive {
    fn write(&self) -> File {
        let tmp_archive_dir = ".kanben-archive";
        let target_dir = Path::new("/tmp");
        let inside_target_dir = target_dir.join(&tmp_archive_dir);
        

        info!("copying backup dir to /tmp");
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = true;

        copy(&self.path, &inside_target_dir, &options)
            .expect("unable to copy config directory");

        info!("directory copied to /tmp");
        info!("creating archive file");

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_millis();
        let filepath = format!("/tmp/kb-{}.archive", timestamp);

        let file = self.get_new_file(&filepath);
        let zip_file = file.try_clone().unwrap();
        let mut zip = ZipWriter::new(zip_file);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.set_comment("kanben archive");

        let prefix = "/tmp/.kanben-archive";

        info!("starting zip...");
        let mut buffer = Vec::new();
        for entry in WalkDir::new(&prefix)
            .into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            info!("starting on {}", &path.display());
            let name = path.strip_prefix(
                Path::new(prefix)
            ).expect("unable to strip prefix");

            if path.is_file() {
                info!("adding file {:?} as {:?} ...", path, name);
                let _ = zip.start_file_from_path(name, options);
                let mut f = File::open(path)
                    .expect("unable to open path");

                let _ = f.read_to_end(&mut buffer);
                let _ = zip.write_all(&*buffer);
                buffer.clear();
            } else if name.as_os_str().len() != 0 {
                info!("adding dir {:?} as {:?} ...", path, name);
                let _ = zip.add_directory_from_path(name, options);
            }
            info!("{}", entry.path().display());
        }

        let _ = zip.finish();

        info!("zip complete. Begin cleanup");

        remove_dir_all(&inside_target_dir)
            .expect("unable to clean up");


        info!("cleanup complete!");

        File::open(&filepath).unwrap()
    }
}
