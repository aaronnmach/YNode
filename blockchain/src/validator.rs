pub struct Validator {
    pub id: String, // Unique identifier (could be a public key in a real system)
    pub stake: u64, // The amount of tokens staked by the validator
}

impl Validator {
    pub fn new(id: String, stake: u64) -> Self {
        Validator { id, stake }
    }
}
