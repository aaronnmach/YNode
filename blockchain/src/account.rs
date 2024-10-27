use crate::coin::Coin;
use crate::transaction::Transaction;
use crate::Blockchain;

pub struct Account {
    pub public_key: String,
    pub coins: Coin,
}

impl Account {
    pub fn new(public_key: String, initial_balance: u64) -> Self {
        Account {
            public_key,
            coins: Coin::new(initial_balance),
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.coins.get_balance()
    }

    pub fn send(&mut self, recipient: &mut Account, amount: u64, blockchain: &mut Blockchain) -> Result<Transaction, &'static str> {
        if self.get_balance() < amount {
            return Err("Insufficient Balance");
        }
        
        let transaction1 = Transaction::new(
            self.public_key.clone(), // Clone the sender's public key
            recipient.public_key.clone(), // Clone the recipient's public key
            amount,
        );
        
        self.coins.add(amount);
        recipient.coins.subtract(amount);
        
        blockchain.add_block(vec![transaction1.clone()]);
        
        Ok(transaction1)
    }
}