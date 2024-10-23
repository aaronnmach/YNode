use crate::validator::Validator;
use rand::Rng;

pub struct PoS {
    pub validators: Vec<Validator>,
}

impl PoS {
    pub fn new() -> Self {
        PoS {
            validators: Vec::new(),
        }
    }

    // Add a validator to the system
    pub fn add_validator(&mut self, id: String, stake: u64) {
        let validator = Validator::new(id, stake);
        self.validators.push(validator);
    }

    // Choose a validator to add the next block, weighted by stake
    pub fn choose_validator(&self) -> Option<&Validator> {
        if self.validators.is_empty() {
            return None;
        }

        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();

        // Choose a validator randomly, weighted by stake
        let mut rng = rand::thread_rng();
        let mut choice = rng.gen_range(0..total_stake);

        for validator in &self.validators {
            if choice < validator.stake {
                return Some(validator);
            }
            choice -= validator.stake;
        }

        None // Default return value in case of an error
    }
}
