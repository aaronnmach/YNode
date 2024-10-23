use crate::pos::PoS;
use crate::block::Block;

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

    pub fn add_block(&mut self, validator_id: &str) {
        let previous_hash = self.chain.last().map_or("0".to_string(), |block| block.hash.clone());
        let new_block = Block::new(self.chain.len() as u32, previous_hash, 0);
        self.chain.push(new_block);

        // For now, manually penalize a validator if needed (logic to be extended)
        if validator_id == "malicious_validator_id" {
            println!("Malicious activity detected for validator {}", validator_id);
            self.pos.penalize_validator(validator_id, 50);  // Penalize by removing 50 tokens
        }
    }
}
