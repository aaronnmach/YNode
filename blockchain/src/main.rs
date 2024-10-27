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
    // Create two accounts with initial balances
    let mut sender = Account::new(1000); // Sender with 1000 coins
    let mut recipient = Account::new(500); // Recipient with 500 coins
    
    // Initialize the blockchain
    let mut blockchain = Blockchain::new();

    // Amount to send
    let amount_to_send = 200;

    // Attempt to send coins
    match sender.send(&mut recipient, amount_to_send, &mut blockchain) {
        Ok(transaction) => {
            println!("Transaction Successful!");
            println!("Sender Key: {:?}", sender.public_key);
            println!("Receiver Key: {:?}", recipient.public_key);
            println!("Amount Sent: {}", amount_to_send);
            println!("Transaction ID (actionID): {}", transaction.actionID);
            println!("Sender New Balance: {}", sender.get_balance());
            println!("Recipient New Balance: {}", recipient.get_balance());
        }
        Err(e) => {
            println!("Transaction Failed: {}", e);
        }
    }
    println!("------------------"); 
    blockchain.print_chain();
}