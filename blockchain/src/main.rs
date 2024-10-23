mod block;
mod blockchain;
mod pos;
mod validator;

use blockchain::Blockchain;
use validator::Validator;

fn main() {
    // Initialize the blockchain
    let mut blockchain = Blockchain::new();

    // Create validators
    let validator1 = Validator::new("validator1".to_string(), 100);
    let malicious_validator = Validator::new("malicious_validator_id".to_string(), 200);

    // Add validators to the PoS system
    blockchain.pos.add_validator(validator1);
    blockchain.pos.add_validator(malicious_validator);

    // Add a block and simulate a malicious validator action
    blockchain.add_block("malicious_validator_id");

    // Print the validators to see their updated stakes
    for validator in &blockchain.pos.validators {
        println!("Validator: {}, Stake: {}, Penalized: {}", validator.id, validator.stake, validator.penalized);
    }
}
