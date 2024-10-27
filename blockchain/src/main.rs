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
    let mut sender = Account::new("234".to_string(), 100);
    let mut receiver = Account::new("235".to_string(), 50);
    let amount = 100;
    
    match sender.send(&mut receiver, amount, &mut blockchain) {
        Ok(transaction) => {
            // Print confirmation message with all transaction details
            println!(
                "Transaction Successful!\n\
                From: {}\n\
                To: {}\n\
                Amount Sent: {}\n\
                Transaction ID: {}\n",
                transaction.senderKey,
                transaction.receiverKey,
                transaction.amount,
                transaction.actionID
            );
        },
        Err(e) => {
            println!("Transaction failed: {}", e);
        }
    }


    // Print the blockchain for verification
    for block in blockchain.chain.iter() {
        println!("{}", block);
    }

}