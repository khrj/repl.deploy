use std::time::{SystemTime, UNIX_EPOCH};

pub fn is_older_than_fifteen_seconds(ts: u128) -> bool {
    ts < (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - 15000)
}
