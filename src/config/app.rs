use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn get() -> AppConfig {
        let content = fs::read_to_string("config.json").expect("cannot get config data");
        serde_json::from_str(&content).expect("cannot get config data")
    }
}
