use std::fs::File;
use log::{info, error};

mod client;
pub use client::Client;

// this is the object that provides a friendly interface to
// the command controller

pub trait Web {
    fn send_backup(
        &mut self, filename: &str, file: File
    ) -> Result<(), ()>;
}

pub struct WebClient {
    client: Client
}

impl WebClient {
    pub fn new(client: Client) -> WebClient {
        WebClient{
            client
        }
    }
}

impl Web for WebClient {
    fn send_backup(
        &mut self,
        filename: &str,
        file: File
    ) -> Result<(), ()>{
        info!("beginning backup: [{}]", filename);
        let res_result = self.client.begin_backup(filename);
        if res_result.is_err() {
            // todo what happens in failure?
            return Err(());
        }

        let response = res_result.unwrap();
        info!("msg from signed url: {}", response.message);
        let r = self.client.put_backup(&response.url, file);
        if r.is_ok() {
            info!("file upload successful");
            Ok(())
        } else {
            error!("response from file put: {}", r.err().unwrap());
            Err(())
        }
    }
}
