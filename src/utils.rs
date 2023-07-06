use std::time::{SystemTime, UNIX_EPOCH};

pub type Timestamp = u64;

pub fn timestamp() -> Timestamp {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}