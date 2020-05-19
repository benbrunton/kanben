use kv::{Store as KvStore, Config, Bucket, Json, Iter};
use dirs::home_dir;
use crate::opts::*;

pub struct Store {
    kv_store: KvStore
}

impl Store {
    pub fn new() -> Store {
        let home_path_bfr = home_dir().unwrap();
        let home_path = home_path_bfr.to_str().unwrap();
        let cfg_location = format!("{}{}", home_path, "/.kanben");
        let mut cfg = Config::new(&cfg_location);

        let kv_store = KvStore::new(cfg)
            .expect("unable to open store");

        Store{ kv_store }

    }

    pub fn get_all(&self) -> Iter<String, Json<Task>> {
        let bucket = self.get_bucket();
        bucket.iter()
    }

    pub fn set(&mut self, key: &str, value: Task) {
        let bucket = self.get_bucket();
        bucket.set(String::from(key), Json(value));
    }

    pub fn get(&self, key: &str) -> Option<Json<Task>> {
        let bucket = self.get_bucket();
        bucket.get(String::from(key)).expect("unable to get")
    }

    fn get_bucket(&self) -> Bucket<String, Json<Task>> {
        self.kv_store.bucket::<String, Json<Task>>(Some("tasks"))
            .expect("unable to get bucket")
    }
}
