use crate::pos::PoS;
use crate::block::Block;
use crate::transaction::Transaction;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pos: PoS,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            pos: PoS::new(),
        }
    }
    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().map_or("0".to_string(), |block| block.hash.clone());
        let timestamp = Self::get_current_timestamp(); // Get current timestamp

        // Create the new block with the transactions
        let new_block = Block::new(self.chain.len() as u32, previous_hash, timestamp, transactions);
        self.chain.push(new_block);

        // Penalize validator for malicious activity (if needed)
        // "malicious_validator_id" is a placeholder
    }
}
