pub struct CoinSupply {
    total_supply: u64,
}

impl CoinSupply {
    // Initialize with a fixed supply
    pub fn new(initial_supply: u64) -> Self {
        CoinSupply {
            total_supply: initial_supply,
        }
    }
    
    // Retrieve total supply
    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }
    
    pub fn mint(&mut self, amount: u64) {
        self.total_supply += amount;
    }
}
