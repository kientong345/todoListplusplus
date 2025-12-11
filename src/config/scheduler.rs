use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SchedulerConfig {
    // pub port: u16,
}

impl SchedulerConfig {
    pub fn get() -> SchedulerConfig {
        // let content = fs::read_to_string("scheduler.json").expect("cannot get config data");
        // serde_json::from_str(&content).expect("cannot get config data")
        todo!()
    }
}
