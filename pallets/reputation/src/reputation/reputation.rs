use std::collections::HashMap;

/// Reputation module responsible for calculating and verifying reputation scores.
pub struct Reputation {
    score: HashMap<String, i32>,
}

impl Reputation {
    /// Initializes a new instance of the Reputation module.
    pub fn new() -> Reputation {
        Reputation {
            score: HashMap::new(),
        }
    }

    /// Calculates the reputation score based on provided data and additional factors.
    /// Incorporates a more complex and realistic scoring algorithm.
    /// 
    /// # Arguments
    /// 
    /// * `data` - A map containing the data used to calculate the score.
    /// * `additional_factors` - A map containing additional factors to be considered in the score calculation.
    /// 
    /// # Returns
    /// 
    /// The calculated reputation score as an integer.
    pub fn calculate_score(&mut self, data: HashMap<String, i32>, additional_factors: HashMap<String, i32>) -> i32 {
        // Enhanced calculation: Incorporate weighted factors in the score calculation.
        let base_score: i32 = data.iter().map(|(_, &value)| value).sum();
        let factor_adjustment: i32 = additional_factors.iter().map(|(_, &value)| value * 2).sum(); // Example of weighted adjustment
        let final_score = base_score + factor_adjustment;

        // Ensure the score stays within a reasonable range
        if final_score < 0 { 0 } else { final_score }
    }

    /// Verifies the reputation score of a DID.
    /// 
    /// # Arguments
    /// 
    /// * `did` - The DID whose score is to be verified.
    /// 
    /// # Returns
    /// 
    /// The reputation score of the DID if found, or 0 if the DID does not exist.
    pub fn verify_score(&self, did: &String) -> i32 {
        *self.score.get(did).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let mut reputation = Reputation::new();
        let mut data = HashMap::new();
        let mut additional_factors = HashMap::new();
        data.insert("key1".to_string(), 10);
        data.insert("key2".to_string(), 20);
        additional_factors.insert("factor1".to_string(), 5);
        additional_factors.insert("factor2".to_string(), 3); // Added more complexity to the test
        assert_eq!(reputation.calculate_score(data, additional_factors), 46); // Adjusted expected value based on new logic
    }

    #[test]
    fn test_verify_score() {
        let mut reputation = Reputation::new();
        let did = "test_did".to_string();
        let score = 42;
        reputation.score.insert(did.clone(), score);
        assert_eq!(reputation.verify_score(&did), score);
    }
}
