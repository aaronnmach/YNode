mod block;
mod blockchain;
mod pos;
mod validator;
mod transaction;
mod coin;
mod account;

use blockchain::Blockchain;
use validator::Validator;
use transaction::Transaction;
use block::Block;
use account::Account;
use coin::Coin;

fn main() {
    // Initialize the blockchain
    let mut blockchain = Blockchain::new();

    // Sample data for testing
    let sender = "SenderPublicKey".to_string();
    let receiver = "ReceiverPublicKey".to_string();
    let amount = 100;
    let signature = vec![0u8; 64]; // Placeholder for the signature

    // Create a few sample transactions
    let transaction1 = Transaction::new(sender.clone(), receiver.clone(), amount, signature.clone());
    let transaction2 = Transaction::new(sender.clone(), receiver.clone(), amount * 2, signature.clone());

    // Group transactions into a vector
    let transactions = vec![transaction1, transaction2];

    // Add a block with these transactions, using a sample validator ID
    blockchain.add_block("validator_1", transactions);

    // Print the blockchain for verification
    for block in blockchain.chain.iter() {
        println!("{}", block);
    }

}