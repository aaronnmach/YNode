use coin::Coin;

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

    pub fn send(&mut self, recipient: &mut Account, amount: u64) -> Result<(), &'static str> {
        self.coins.subtract(amount)?;
        recipient.coins.add(amount);
        Ok(())
    }
}