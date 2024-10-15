use serde::Deserialize;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub message: String,
    pub timestamp: SystemTime,
}
