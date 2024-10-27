use crate::transaction::Transaction;

pub struct Coin {
    pub balance: u64,
}

impl Coin {
    pub fn new(initial_balance: u64) -> Self {
        Coin {
            balance: initial_balance,
        }
    }

    pub fn add(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn subtract(&mut self, amount: u64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient balance")
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}


