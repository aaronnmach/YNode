use sha2::{Sha256, Digest};
use std::fmt;

pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, previous_hash: String, timestamp: u64) -> Self {
        let hash = format!("{}:{}:{}", index, previous_hash, timestamp); // Simplified hash for now
        Block {
            index,
            previous_hash,
            timestamp,
            hash,
        }
    }

    // A simple hash calculation combining the index, previous hash, and timestamp
    pub fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", index, previous_hash, timestamp));
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

// Implementing Display trait for pretty printing
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block[{}]:\n\tHash: {}\n\tPrevious Hash: {}\n\tTimestamp: {}\n", 
               self.index, self.hash, self.previous_hash, self.timestamp)
    }
}
