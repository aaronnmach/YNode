mod blockchain; // Import the blockchain module
mod block;      // Import the block module
mod validator;  // Import the validator module
mod pos;        // Import the PoS module

use pos::PoS;

fn main() {
    // Step 1: Initialize a PoS system
    let mut pos_system = PoS::new();

    // Step 2: Add some validators with different stakes
    pos_system.add_validator(String::from("Alice"), 100);
    pos_system.add_validator(String::from("Bob"), 50);
    pos_system.add_validator(String::from("Charlie"), 200);

    // Step 3: Simulate choosing a validator to propose a block
    if let Some(chosen_validator) = pos_system.choose_validator() {
        println!(
            "Validator chosen to propose the block: {} with stake: {}",
            chosen_validator.id, chosen_validator.stake
        );
    } else {
        println!("No validators available.");
    }

    // Step 4: Repeat to simulate multiple block proposals
    for _ in 0..5 {
        if let Some(chosen_validator) = pos_system.choose_validator() {
            println!(
                "Validator chosen to propose the block: {} with stake: {}",
                chosen_validator.id, chosen_validator.stake
            );
        } else {
            println!("No validators available.");
        }
    }
}
