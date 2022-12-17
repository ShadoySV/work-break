use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub is_working: bool,
    pub plugin_id: i32,
    pub work: Duration,
    pub at: SystemTime,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_working: false,
            plugin_id: Default::default(),
            work: Default::default(),
            at: SystemTime::now(),
        }
    }
}
