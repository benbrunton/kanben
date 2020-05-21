use kv::{Store as KvStore, Config, Bucket, Json};
use dirs::home_dir;
use crate::opts::Task;

pub trait Store {
    fn get_all(&self, bucket: &str) -> Vec<Task>;
    fn get(&self, key: &str, bucket: &str) -> Option<Task>;
    fn set(&mut self, key: &str, value: Task, bucket: &str);
    fn rm(&mut self, key: &str, bucket: &str);
}

pub struct PersistantStore {
    kv_store: KvStore
}

impl PersistantStore {
    pub fn new() -> PersistantStore {
        let home_path_bfr = home_dir().unwrap();
        let home_path = home_path_bfr.to_str().unwrap();
        let cfg_location = format!("{}{}", home_path, "/.kanben");
        let cfg = Config::new(&cfg_location);
        let kv_store = KvStore::new(cfg)
            .expect("unable to open store");

        PersistantStore{ kv_store }
    }

    fn get_bucket(&self, bucket_label: &str
        ) -> Bucket<String, Json<Task>> {
        self.kv_store.bucket::<String, Json<Task>>(
            Some(bucket_label)
        ).expect("unable to get bucket")
    }
}

impl Store for PersistantStore {
    fn get_all(&self, bucket_label: &str) -> Vec<Task> {
        let bucket = self.get_bucket(bucket_label);
        bucket.iter().map(|item| { 
            let task = item.unwrap();
            let json_value = task.value::<Json<Task>>().unwrap();
            json_value.as_ref().to_owned()
        }).collect()
    }

    fn set(&mut self, key: &str, value: Task, bucket_label: &str) {
        let bucket = self.get_bucket(bucket_label);
        let _ = bucket.set(String::from(key), Json(value));
    }

    fn get(&self, key: &str, bucket_label: &str) -> Option<Task> {
        let bucket = self.get_bucket(bucket_label);
        let item_result = bucket.get(String::from(key))
            .expect("unable to connect to kv store");
        match item_result {
            None => None,
            Some(x) => Some(x.as_ref().clone())
        }
    }

    fn rm(&mut self, key: &str, bucket_label: &str) {
        let bucket = self.get_bucket(bucket_label);
        let _ = bucket.remove(String::from(key));
    }
}


