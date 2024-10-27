use sha2::{Sha256, Digest};
use std::fmt;
use crate::transaction::Transaction;
use hex;

pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub next: Option<Box<Block>>,
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
            next: None,
        }
    }

    pub fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, transactions: &[Transaction]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", index, previous_hash, timestamp));
        
        // Include the transaction data in the hash calculation
        for tx in transactions {
            // Convert Vec<u8> to a hex string representation for hashing
            let sender_key_hex = hex::encode(&tx.senderKey);
            let receiver_key_hex = hex::encode(&tx.receiverKey);
            
            hasher.update(format!("{}{}{}", sender_key_hex, receiver_key_hex, tx.amount));
        }
        
        let result = hasher.finalize();
        format!("{:x}", result)
    }    
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        // You might want to add conditions, e.g., maximum transactions per block
        // if self.transactions.len() >= MAX_TRANSACTIONS {
        //     return Err("Transaction limit reached for this block");
        // }
        
        // Add the transaction to the block's transaction list
        self.transactions.push(transaction);
        Ok(())
    }
}

// Implementing Display trait for pretty printing
    impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Block[{}]:\n\tHash: {}\n\tPrevious Hash: {}\n\tTimestamp: {}\n\tTransactions: {}\n", 
                self.index, self.hash, self.previous_hash, self.timestamp, self.transactions.len())
        }
    }
    
