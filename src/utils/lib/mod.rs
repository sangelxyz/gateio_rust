use std::time::{SystemTime, UNIX_EPOCH};

// Create a linux time stamp with the current time.
pub fn time_stamp() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
