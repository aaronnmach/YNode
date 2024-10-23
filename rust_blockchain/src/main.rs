use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = get_current_time();
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

fn get_current_time() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis()
}

fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> String {
    let mut hasher = Sha256::new();
    let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
