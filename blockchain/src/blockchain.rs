use crate::pos::PoS;
use crate::block::Block;
use crate::transaction::Transaction;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub head: Option<Box<Block>>,
    pub count: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"), 0, Vec::new());
        Blockchain {
            head: Some(Box::new(genesis_block)), // Initialize with the genesis block
            count: 1,
        }
    }
    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    pub fn add_block(&mut self, new_block: Block) {
        // If the blockchain is empty, set the head to the new block
        if self.head.is_none() {
            self.head = Some(Box::new(new_block));
        } else {
            // Traverse to the last block
            let mut current = self.head.as_mut(); // Get mutable reference to the head

            while let Some(mut block) = current.take() {
                // If there is no next block, we can add the new block here
                if block.next.is_none() {
                    block.next = Some(Box::new(new_block));
                    current = Some(block); // Restore current
                    break;
                } else {
                    // Move to the next block
                    current = block.next.as_mut(); // Update current to the next block
                }
            }
        }

        // Increment the block count
        self.count += 1;
    }
    pub fn print_chain(&self) {
        let mut current = self.head.as_ref(); // Start with the head of the chain
        while let Some(block) = current {
            println!(
                "Block Index: {}\nPrevious Hash: {}\nTimestamp: {}\nTransactions: {:?}\n",
                block.index,
                block.previous_hash,
                block.timestamp,
                block.transactions.len(),
            );
            current = block.next.as_ref(); // Move to the next block
        }
    }
}
