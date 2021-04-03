use std::time::{SystemTime, UNIX_EPOCH};

pub fn is_older_than_fifteen_seconds(ts: f64) -> bool {
    ts < (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 15) as f64
}
