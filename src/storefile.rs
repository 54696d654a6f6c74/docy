use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs;

const FILE_PATH: &str = ".docy.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capture {
    pub start: usize,
    pub end: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub captrues: Vec<Capture>,
    pub path: OsString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreFile {
    pub last_action: Option<Action>,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Ex,
    In,
}

impl StoreFile {
    pub fn new() -> Self {
        return Self {
            last_action: None,
            files: vec![],
        };
    }

    pub fn load() -> anyhow::Result<Self> {
        let store_file_data = fs::read_to_string(FILE_PATH)?;
        let store_data = serde_json::from_str(&store_file_data)?;

        return Ok(store_data);
    }

    pub fn commit(&self) -> anyhow::Result<()> {
        let stringified = serde_json::to_string(&self)?;

        fs::write(FILE_PATH, &stringified)?;

        return Ok(());
    }
}
