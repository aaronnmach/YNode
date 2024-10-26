use sha2::{Sha256, Digest};
use std::fmt;
use crate::transaction::Transaction;

pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, previous_hash: String, timestamp: u64, transactions: Vec<Transaction>) -> Self {
        let hash = Self::calculate_hash(index, &previous_hash, timestamp, &transactions);
        Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            hash,
        }
    }
    // A simple hash calculation combining the index, previous hash, and timestamp
    pub fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, transactions: &[Transaction]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", index, previous_hash, timestamp));
        
        // Include the transaction data in the hash calculation
        for tx in transactions {
            hasher.update(format!("{}{}{}", tx.senderKey, tx.receiverKey, tx.amount));
        }
        
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

// Implementing Display trait for pretty printing
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block[{}]:\n\tHash: {}\n\tPrevious Hash: {}\n\tTimestamp: {}\n\tTransactions: {}\n", 
               self.index, self.hash, self.previous_hash, self.timestamp, self.transactions.len())
    }
}
