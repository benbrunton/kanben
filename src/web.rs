mod client;
pub use client::Client;

// this is the object that provides a friendly interface to
// the command controller

pub trait Web {
    fn send_backup(&mut self) -> Result<(), ()>;
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
    // todo: verbose logging
    fn send_backup(&mut self) -> Result<(), ()>{
        // first we're going to hit the endpoint
        let res_result = self.client.begin_backup();
        match res_result {
            Ok(r) => {
                println!("{:?}", r)
            },
            _ => ()
        }
        Ok(())
    }
}
