use crate::block::Block;
use crate::pos::PoS;
use std::time::{SystemTime, UNIX_EPOCH}; // Import system time for timestamp

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pos: PoS, // Proof of Stake system
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: Vec::new(),
            pos: PoS::new(),
        }
    }

    pub fn add_block(&mut self) {
        if let Some(validator) = self.pos.choose_validator() {
            let previous_hash = if self.chain.is_empty() {
                String::from("0") // Genesis block
            } else {
                self.chain.last().unwrap().hash.clone()
            };

            // Get current timestamp as u64
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            let block = Block::new(self.chain.len() as u32, previous_hash, timestamp); // Pass the timestamp
            println!("Block added by validator: {}", validator.id);
            self.chain.push(block);
        } else {
            println!("No validators available to add a block.");
        }
    }

    pub fn add_validator(&mut self, id: String, stake: u64) {
        self.pos.add_validator(id, stake);
    }
}
