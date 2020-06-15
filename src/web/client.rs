use reqwest::{blocking::Client as ReqwestClient, Error};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs::File;

#[derive(Debug, Serialize)]
pub struct BeginBackupRequest{
    filename: String
}

#[derive(Debug, Deserialize)]
pub struct BeginBackupResponse {
    pub message: String,
    pub url: String
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

    pub fn begin_backup(&self, filename: &str) 
        -> Result<BeginBackupResponse, Error> {
        let request = BeginBackupRequest {
            filename: filename.to_owned()
        };

        self.post_json_or_error(
            "https://kan.benbru.com/begin-backup",
            Some(request)
        )
    }

    pub fn put_backup(&self, url: &str, file: File) 
        -> Result<(), Error> {
        self.put_file_or_error(url, file)
    }

    fn post_json_or_error<T: DeserializeOwned, B: Serialize>(
        &self, path: &str, body: Option<B>) -> Result<T, Error> {
        let mut req = self.req_client.post(path);

        if body.is_some() {
            req = req.json(&body.unwrap());
        }

        let res_result = req.send();

        match res_result {
            Ok(r) => {
                let response_body: T = r.json()?;
                Ok(response_body)
            },
            Err(err) => Err(err)
        }
    }

    fn put_file_or_error(&self, path: &str, body: File)
        -> Result<(), Error> {
        let mut req = self.req_client.put(path);

        req = req.body(body);

        let res_result = req.send();

        match res_result {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }

    }
}
