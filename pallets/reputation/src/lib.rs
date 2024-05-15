// lib.rs
use std::collections::HashMap;

extern crate ipfs;
extern crate substrate_node_template;
extern crate zk_snarks;

mod did;
mod reputation;
mod data_aggregation;
mod smart_contracts;
mod privacy;

pub struct Lib {
    did_module: did::DID,
    reputation_module: reputation::Reputation,
    data_aggregation_module: data_aggregation::DataAggregation,
    smart_contracts_module: smart_contracts::SmartContracts,
    privacy_module: privacy::Privacy,
}

impl Lib {
    pub fn new() -> Self {
        Self {
            did_module: did::DID::default(),
            reputation_module: reputation::Reputation::default(),
            data_aggregation_module: data_aggregation::DataAggregation::default(),
            smart_contracts_module: smart_contracts::SmartContracts::default(),
            privacy_module: privacy::Privacy::default(),
        }
    }

    pub fn generate_did(&mut self) -> String {
        self.did_module.generate_did()
    }

    pub fn register_did(&mut self, did: String) -> bool {
        self.did_module.register_did(did)
    }

    pub fn update_did(&mut self, did: String) -> bool {
        self.did_module.update_did(did)
    }

    pub fn revoke_did(&mut self, did: String) -> bool {
        self.did_module.revoke_did(did)
    }

    pub fn calculate_score(&mut self, data: HashMap<String, i32>) -> i32 {
        self.reputation_module.calculate_score(data)
    }

    pub fn verify_score(&mut self, did: String) -> i32 {
        self.reputation_module.verify_score(did)
    }

    pub fn aggregate_data(&mut self) -> HashMap<String, i32> {
        self.data_aggregation_module.aggregate_data()
    }

    pub fn create_loan_contract(&mut self) -> bool {
        self.smart_contracts_module.create_loan_contract()
    }

    pub fn verify_reputation_contract(&mut self, did: String) -> bool {
        self.smart_contracts_module.verify_reputation_contract(did)
    }

    pub fn manage_loan_contract(&mut self) -> bool {
        self.smart_contracts_module.manage_loan_contract()
    }

    pub fn encrypt_data(&mut self, data: String) -> String {
        self.privacy_module.encrypt_data(data)
    }

    pub fn decrypt_data(&mut self, data: String) -> String {
        self.privacy_module.decrypt_data(data)
    }
}

// Ensure to include Default trait implementations for each module to facilitate the `new` method in `Lib`.
// This approach encourages modularity and ease of testing for each component.

// The Cargo.toml must be updated to include dependencies like `ipfs-api`, `substrate-api-client`, and any `zk-snarks` compatible library available for Rust.
// Example:
// [dependencies]
// ipfs-api = "0.6.0"
// substrate-api-client = "0.9.0"
// zk-snarks = "1.0.0"
// This ensures that all external dependencies are correctly declared and versioned.
