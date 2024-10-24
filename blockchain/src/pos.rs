use crate::validator::Validator;
use rand::Rng;

pub struct PoS {
    pub validators: Vec<Validator>,
}

impl PoS {
    pub fn new() -> Self {
        Self { validators: Vec::new() }
    }

    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.push(validator);
    }

    // Select a validator based on their stake
    pub fn select_validator(&self) -> Option<&Validator> {
        if self.validators.is_empty() {
            return None;
        }

        // Total stake of all validators
        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();
        let mut rng = rand::thread_rng();
        let mut selected_value = rng.gen_range(0..total_stake);

        for validator in &self.validators {
            if selected_value < validator.stake {
                return Some(validator);
            }
            selected_value -= validator.stake;
        }
        None
    }

    // Penalize a validator for malicious behavior
    pub fn penalize_validator(&mut self, validator_id: &str, penalty_amount: u64) {
        if let Some(validator) = self.validators.iter_mut().find(|v| v.id == validator_id) {
            validator.penalize(penalty_amount);
        } else {
            println!("Validator with ID {} not found.", validator_id);
        }
    }
    
}
