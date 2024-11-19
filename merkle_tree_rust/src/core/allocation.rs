use serde::Deserialize;
use starknet::core::types::Felt;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Allocation {
    pub address: String,
    pub amount: u64,
    pub timestamp: String,
    pub id: u64,
}

impl Hash for Allocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
        self.amount.hash(state);
        self.timestamp.hash(state);
        self.id.hash(state);
    }
}

impl Allocation {
    /// Converts the allocation fields into Felt values for hashing.
    pub fn to_felts(&self) -> Result<(Felt, Felt, Felt, Felt), String> {
        let felt_address = Felt::from_hex(&self.address).map_err(|_| "Invalid address")?;
        let felt_amount = u64_to_felt(self.amount);
        let felt_timestamp = Felt::from_hex(&self.timestamp).map_err(|_| "Invalid timestamp")?;
        let felt_id = u64_to_felt(self.id);

        Ok((felt_address, felt_amount, felt_timestamp, felt_id))
    }
}

/// Utility function to convert a u64 to a Felt.
pub fn u64_to_felt(value: u64) -> Felt {
    let mut bytes = [0u8; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    Felt::from_bytes_be(&bytes)
}
