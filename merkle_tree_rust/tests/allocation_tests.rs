#[cfg(test)]
mod tests {
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
    fn test_u64_to_felt_conversion() {
        let value = 150;
        let felt = u64_to_felt(value);
        let mut value_bytes = [0u8; 8];
        value_bytes.copy_from_slice(&value.to_be_bytes());

        // Ensure the serialized bytes of `felt` match the original value.
        let felt_bytes = felt.to_bytes_be();
        assert_eq!(felt_bytes[24..], value_bytes);
    }
}
