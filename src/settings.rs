use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub root_dir: String,
    pub exclude_dir: Vec<String>,
    pub exclude_file: Vec<String>,
}

impl Settings {
    pub fn new(data_str: &str) -> Result<Self> {
        return serde_json::from_str(data_str);
    }

    fn stringified(&self) -> Result<String> {
        return serde_json::to_string_pretty(self);
    }

    pub fn load_settings(path: &str) -> Self {
        let target = match fs::read_to_string(path) {
            Err(_) => {
                let content = Settings::default().stringified().unwrap();

                fs::write(path, content.as_bytes())
                    .expect("Failed to create default settings file");

                return Settings::default();
            }
            Ok(val) => val,
        };

        return Settings::new(&target).expect("Failed to parse data from settings file!");
    }
}

impl Default for Settings {
    fn default() -> Self {
        return Settings {
            root_dir: String::from("."),
            exclude_dir: vec![],
            exclude_file: vec![],
        };
    }
}
