#[cfg(test)]
mod tests {
    use std::{
        hash::{DefaultHasher, Hash, Hasher},
        u64,
    };

    use merkle_tree::core::allocation::{u64_to_felt, Allocation};
    use starknet::core::types::Felt;

    #[test]
    fn test_allocation_to_felts() {
        let allocation = Allocation {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: 150,
            timestamp: "0x2".to_string(),
            id: 1,
        };

        let felts = allocation.to_felts().expect("Failed to convert to felts");
        assert_eq!(
            felts.0,
            Felt::from_hex("0x1234567890abcdef1234567890abcdef12345678").unwrap()
        );
        assert_eq!(felts.1, u64_to_felt(150));
        assert_eq!(felts.2, Felt::from_hex("0x2").unwrap());
        assert_eq!(felts.3, u64_to_felt(1));
    }

    #[test]
    fn test_allocation_invalid_address() {
        let allocation = Allocation {
            address: "0x12939jojdo30".to_string(),
            amount: 150,
            timestamp: "0x".to_string(),
            id: 1,
        };
        let felts = allocation.to_felts();
        assert!(felts.is_err());
        assert_eq!(felts.unwrap_err(), "Invalid address");
    }

    #[test]
    fn test_allocation_invalid_timestamp() {
        let allocation = Allocation {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: 150,
            timestamp: "0x38djij".to_string(),
            id: 1,
        };
        let felts = allocation.to_felts();
        assert!(felts.is_err());
        assert_eq!(felts.unwrap_err(), "Invalid timestamp");
    }

    #[test]
    fn test_u64_to_felt_conversion() {
        let value = 150;
        let felt = u64_to_felt(value);
        let mut value_bytes = [0u8; 8];
        value_bytes.copy_from_slice(&value.to_be_bytes());

        // Ensure the serialized bytes of `felt` match the original value.
        let felt_bytes = felt.to_bytes_be();
        assert_eq!(felt_bytes[24..], value_bytes);
    }

    #[test]
    fn test_u64_edge_case() {
        // validation for the minimal edge case
        let zero_value = 0;
        let felt = u64_to_felt(zero_value);
        assert!(matches!(felt, Felt { .. }));

        // validation for max value
        let max_value = u64::MAX;
        let felt = u64_to_felt(max_value);
        assert!(matches!(felt, Felt { .. }));
    }

    #[test]
    fn test_allocation_hash() {
        let allocation = Allocation {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: 150,
            timestamp: "0x2".to_string(),
            id: 1,
        };
        let mut state = DefaultHasher::new();
        allocation.hash(&mut state);
        let hash = state.finish();

        assert_ne!(hash, 0, "Hash should not be zero");
        assert_eq!(std::mem::size_of_val(&hash), 8, "Hash should be u64");

        let mut other_state = DefaultHasher::new();
        allocation.hash(&mut other_state);
        let other_hash = other_state.finish();

        assert_eq!(hash, other_hash, "Hash should be the same");
    }
}
