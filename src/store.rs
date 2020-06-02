use kv::{Bucket, Json, Codec};

pub trait Store <T>{
    fn get_all(&self) -> Vec<T>;
    fn get(&self, key: &str) -> Option<T>;
    fn set(
        &mut self,
        key: &str,
        value: T
    );
    fn rm(&mut self, key: &str);
}

pub struct PersistantStore <'a, T: serde::Serialize + serde::de::DeserializeOwned> {
    bucket: &'a Bucket<'a, String, Json<T>>
}

impl <'a, T: serde::Serialize + serde::de::DeserializeOwned> PersistantStore <'a, T> {
    pub fn new(bucket: &'a Bucket<String, Json<T>>) -> PersistantStore<'a, T> {

        PersistantStore{ bucket }
    }
}

impl <'a, 
    T: serde::Serialize + serde::de::DeserializeOwned
> Store<T> for PersistantStore<'a, T> {
    fn get_all(&self) -> Vec<T> {
        self.bucket.iter().map(|item| { 
            let task = item.unwrap();
            let json_value = task.value::<Json<T>>().unwrap();
            json_value.to_inner()
        }).collect()
    }

    fn set(&mut self, key: &str, value: T) {
        let _ = self.bucket.set(String::from(key), Json(value));
    }

    fn get(&self, key: &str) -> Option<T> {
        let item_result = self.bucket.get(String::from(key))
            .expect("unable to connect to kv store");
        match item_result {
            None => None,
            Some(x) => {
                Some(x.to_inner())
            }
        }
    }

    fn rm(&mut self, key: &str) {
        let _ = self.bucket.remove(String::from(key));
    }
}


