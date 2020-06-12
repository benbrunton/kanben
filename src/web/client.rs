use reqwest::{blocking::Client as ReqwestClient, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BeginBackupResponse {
    message: String,
    url: String
}

// this is the struct that provides a mapping
// to the web api and encapsulates all the http client domain
pub struct Client {
    req_client: ReqwestClient
}

impl Client {
    pub fn new(req_client: ReqwestClient) -> Client {
        Client { req_client }
    }

    pub fn begin_backup(&mut self) 
        -> Result<BeginBackupResponse, Error> {
        // first we're going to hit the endpoint
        let res_result = self.req_client.post(
            "https://kan.benbru.com/begin-backup"
        ).send();

        match res_result {
            Ok(r) => {
                let response_body: BeginBackupResponse = r.json()?;
                Ok(response_body)
            },
            Err(err) => Err(err)
        }

    }
}
