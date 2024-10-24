use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Transaction {
    pub senderKey: String, 
    pub receiverKey: String,
    pub amount: u64,
    pub timestamp: u64,
    pub actionID: String,
    pub signature: Vec<u8>
}

impl Transaction{   
    // for creating a new transaction
    pub fn new(sender: String, receiver: String, amount: u64, signature: Vec<u8>) -> Self{
        let timestamp = Self::get_current_timestamp();
        let actionID = Transaction::calculate_tx_id(&sender, &receiver, amount, timestamp);
        Transaction {
            senderKey: sender,
            receiverKey: receiver,
            amount,
            timestamp,
            actionID,
            signature,
        }
    }
    // generates a unique hash for the transaction based on sender, receipient, amount, timestamp
    pub fn calculate_tx_id(senderKey: &str, receiverKey: &str, amount: u64, timestamp: u64) -> String {
        let tx_data = format!("{}:{}:{}:{}", senderKey, receiverKey, amount, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(tx_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)  // Convert to hex string
    }
    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
    /*
    // THESE ARE CURRENTLY UNUSABLE
    // _______________________________________________
    
    // signs transaction to prove authenticity
    // NOTE: needs to be signed with cryptographic library
    pub fn sign_transaction(&mut self, private_key: &PrivateKey) {
        let tx_data = format!("{}:{}:{}:{}", self.sender, self.recipient, self.amount, self.timestamp);
        self.signature = private_key.sign(tx_data.as_bytes());  // .sign is a placeholder
    }
    // verifies transaction using senders key
    // NOTE: needs to be verified with cryptographic library
    pub fn verify_transaction(&self, public_key: &PublicKey) -> bool {
        let tx_data = format!("{}:{}:{}:{}", self.sender, self.recipient, self.amount, self.timestamp);
        public_key.verify(tx_data.as_bytes(), &self.signature) // .verify is a placeholder
    }
    pub fn is_valid(&self, blockchain: &Blockchain) -> bool {
        // Example: Check if sender has enough funds, and transaction signature is valid
        if blockchain.get_balance(&self.sender) < self.amount {
            println!("Insufficient funds");
            return false;
        }

        // Verify the signature
        let sender_public_key = blockchain.get_public_key(&self.sender);  // Assume a method to get public key
        if !self.verify_transaction(&sender_public_key) {
            println!("Invalid signature");
            return false;
        }

        true
    }*/
}