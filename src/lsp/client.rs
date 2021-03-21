use super::raw_client::RawClient;
use crate::lsp;
use serde::Serialize;
use std::{error::Error, path::PathBuf};

pub struct Client {
    pub work_dir: PathBuf,
    pub raw: RawClient,
}

impl Client {
    pub fn new(bin_path: PathBuf, work_dir: PathBuf) -> Client {
        return Client {
            work_dir,
            raw: RawClient::new(bin_path),
        };
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.raw.run()?;

        let req = make_init(&self.work_dir);

        match to_json(&req) {
            Ok(data) => self.raw.send(data),
            Err(e) => eprintln!("Error while serializing: {}", e),
        }

        Ok(())
    }
}

fn to_json<T: Serialize>(t: &T) -> serde_json::Result<Vec<u8>> {
    let data = serde_json::to_string(&t)?;

    Ok(data.into())
}

fn make_init(work_dir: &PathBuf) -> lsp::Request<lsp::InitializeParams> {
    lsp::Request::<lsp::InitializeParams> {
        id: lsp::RequestId::Number(0),
        method: "initialize".into(),
        params: Some(lsp::InitializeParams {
            rootPath: Some(work_dir.to_string_lossy().into()),
            ..Default::default()
        }),
    }
}
