use crate::coin::Coin;
use crate::transaction::Transaction;
use crate::Blockchain;
use ring::signature::{self, Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::rand::SystemRandom;

pub struct Account {
    pub public_key: Vec<u8>,
    pub coins: Coin,
    private_key: signature::Ed25519KeyPair,
}

impl Account {
    // Going to implement system for creating keys later
    pub fn new(initial_balance: u64) -> Self {
        let rng = SystemRandom::new();
        let private_key = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(private_key.as_ref()).unwrap();

        let public_key = key_pair.public_key().as_ref().to_vec();

        Account {
            public_key,
            coins: Coin::new(initial_balance),
            private_key: key_pair, // Store the key pair securely within the struct
        }
    }
    pub fn sign_transaction(&self, message: &[u8]) -> Vec<u8> {
        self.private_key.sign(message).as_ref().to_vec()
    }

    pub fn verify_transaction(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), &'static str> {
        let public_key = UnparsedPublicKey::new(&ED25519, public_key);

        public_key.verify(message, signature).map_err(|_| "Verification failed")
    }
    
    pub fn get_balance(&self) -> u64 {
        self.coins.get_balance()
    }
    pub fn send(
        &mut self,
        recipient: &mut Account,
        amount: u64,
        blockchain: &mut Blockchain
    ) -> Result<Transaction, &'static str> {
        // Check if the account balance is sufficient
        if self.get_balance() < amount {
            return Err("Insufficient Balance");
        }
    
        // Create the transaction (including actionID)
        let transaction = Transaction::new(self.public_key.clone(), recipient.public_key.clone(), amount);
    
        // Retrieve the actionID (message) from the transaction
        let action_id = &transaction.actionID;
    
        // Sign the transaction message
        let signature = self.sign_transaction(action_id.as_bytes());
    
        // Verify the signature
        if let Err(e) = Account::verify_transaction(&self.public_key, action_id.as_bytes(), &signature) {
            return Err(e);
        }
    
        // Adjust balances
        self.coins.subtract(amount)?;
        recipient.coins.add(amount);
    
        // Add the transaction to a new block in the blockchain
        blockchain.add_block(vec![transaction.clone()]);
    
        // Return the transaction as confirmation
        Ok(transaction)
    }
    
}