pub struct Validator {
    pub id: String, // Unique identifier (could be a public key in a real system)
    pub stake: u64, // The amount of tokens staked by the validator
    pub penalized: bool,
}

impl Validator {
    pub fn new(id: String, stake: u64) -> Self {
        Self {
            id,
            stake,
            penalized: false,
        }
    }
    pub fn penalize(&mut self, penalty_amount: u64) {
        // Ensure that the penalty does not exceed the validator's total stake
        if self.stake >= penalty_amount {
            self.stake -= penalty_amount;
        } else {
            self.stake = 0;
        }
        self.penalized = true;
        println!("Validator {} has been penalized. New stake: {}", self.id, self.stake);
    }

    // Reset the penalty if needed (for testing or future implementation)
    pub fn reset_penalty(&mut self) {
        self.penalized = false;
    }
}
