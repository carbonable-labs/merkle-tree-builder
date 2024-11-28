use merkle_tree::core::{allocation::Allocation, merkle_tree::MerkleTree};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn load_mock_data(filename: &str) -> Vec<Allocation> {
        let path = Path::new("data").join(filename);
        let data =
            fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", filename));
        let json: Value =
            serde_json::from_str(&data).unwrap_or_else(|_| panic!("Failed to parse JSON"));

        json.as_array()
            .unwrap_or_else(|| panic!("JSON is not an array"))
            .iter()
            .map(|item| Allocation {
                address: item["address"].as_str().unwrap().to_string(),
                amount: item["amount"].as_u64().unwrap(),
                timestamp: item["timestamp"].as_str().unwrap().to_string(),
                id: item["id"].as_u64().unwrap(),
            })
            .collect()
    }

    #[test]
    fn test_first_wave_specific_allocations() {
        let first_wave = load_mock_data("mock_allocations_first_wave.json");
        let tree = MerkleTree::new(first_wave.clone());

        // Save the initial root hash for comparison
        let initial_root = tree.root.value;

        // Test specific first wave allocations
        let test_cases = [
            (
                "0x1234567890abcdef1234567890abcdef12345678",
                150u64,
                "0x2",
                1u64,
            ),
            (
                "0xabcdefabcdefabcdefabcdefabcdefabcdef1234",
                200u64,
                "0x1",
                1u64,
            ),
        ];

        for (address, amount, timestamp, id) in test_cases.iter() {
            let result = tree.build_address_calldata(address, *amount, timestamp, *id);
            assert!(
                result.is_ok(),
                "Failed to generate proof for first wave allocation: {}",
                address
            );

            // Verify proof format and content
            let proof = result.unwrap();
            assert!(
                proof.len() > 4,
                "Proof should contain allocation data and hashes"
            );
            assert_eq!(
                proof[0],
                address.to_string(),
                "Address should match in proof"
            );
        }

        // Verify tree integrity
        assert_eq!(
            tree.root.value, initial_root,
            "Tree root should remain unchanged"
        );
    }

    #[test]
    fn test_second_wave_specific_allocations() {
        let second_wave = load_mock_data("mock_allocations_second_wave.json");
        let tree = MerkleTree::new(second_wave.clone());

        // Save initial root for verification
        let initial_root = tree.root.value;

        let test_cases = [
            (
                "0xabcabcabcabcabcabcabcabcabcabcabcabcabc1",
                500u64,
                "0x4",
                1u64,
            ),
            (
                "0x7897897897897897897897897897897897897890",
                400u64,
                "0xA",
                1u64,
            ),
        ];

        for (address, amount, timestamp, id) in test_cases.iter() {
            let result = tree.build_address_calldata(address, *amount, timestamp, *id);
            assert!(
                result.is_ok(),
                "Failed to generate proof for second wave allocation: {}",
                address
            );

            // Verify proof structure
            let proof = result.unwrap();
            assert_eq!(
                proof[0],
                address.to_string(),
                "Address should match in proof"
            );
        }

        // Verify tree integrity
        assert_eq!(
            tree.root.value, initial_root,
            "Tree root should remain unchanged"
        );
    }

    #[test]
    fn test_merged_trees_with_specific_proofs() {
        let first_wave = load_mock_data("mock_allocations_first_wave.json");
        let second_wave = load_mock_data("mock_allocations_second_wave.json");

        // Create and merge trees
        let first_tree = MerkleTree::new(first_wave.clone());
        let first_root = first_tree.root.value;
        let merged_tree = first_tree.merge_merkle_trees(second_wave.clone());

        // Verify merged tree contains more allocations
        assert!(
            merged_tree.get_allocations().len() > first_wave.len(),
            "Merged tree should contain more allocations"
        );

        // Verify merged root is different from first tree's root
        assert_ne!(
            merged_tree.root.value, first_root,
            "Merged tree should have different root hash"
        );

        let test_cases = [
            // From first wave
            (
                "0x1234567890abcdef1234567890abcdef12345678",
                150u64,
                "0x2",
                1u64,
            ),
            // From second wave
            (
                "0xabcabcabcabcabcabcabcabcabcabcabcabcabc1",
                500u64,
                "0x4",
                1u64,
            ),
        ];

        for (address, amount, timestamp, id) in test_cases.iter() {
            let result = merged_tree.build_address_calldata(address, *amount, timestamp, *id);
            assert!(
                result.is_ok(),
                "Failed to generate proof for allocation in merged tree: {}",
                address
            );

            // Verify proof structure
            let proof = result.unwrap();
            assert_eq!(
                proof[0],
                address.to_string(),
                "Address should match in proof"
            );
        }
    }

    #[test]
    fn test_duplicate_addresses() {
        let second_wave = load_mock_data("mock_allocations_second_wave.json");
        let tree = MerkleTree::new(second_wave.clone());
        let initial_root = tree.root.value;

        let test_cases = [
            (
                "0x7897897897897897897897897897897897897890",
                400u64,
                "0xA",
                1u64,
            ),
            (
                "0x7897897897897897897897897897897897897890",
                150u64,
                "0xB",
                2u64,
            ),
        ];

        for (address, amount, timestamp, id) in test_cases.iter() {
            let result = tree.build_address_calldata(address, *amount, timestamp, *id);
            assert!(
                result.is_ok(),
                "Failed to generate proof for duplicate address: {} with amount: {}",
                address,
                amount
            );

            // Verify proof structure
            let proof = result.unwrap();
            assert_eq!(
                proof[0],
                address.to_string(),
                "Address should match in proof"
            );
        }

        // Verify tree integrity maintained
        assert_eq!(
            tree.root.value, initial_root,
            "Tree root should remain unchanged"
        );
    }

    #[test]
    fn test_invalid_allocations() {
        let tree = MerkleTree::new(load_mock_data("mock_allocations_first_wave.json"));
        let initial_root = tree.root.value;

        let test_cases = [
            (
                "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                999999u64,
                "0xff",
                999u64,
            ),
            (
                "0x0000000000000000000000000000000000000000",
                0u64,
                "0x0",
                0u64,
            ),
        ];

        for (address, amount, timestamp, id) in test_cases.iter() {
            let result = tree.build_address_calldata(address, *amount, timestamp, *id);
            assert!(
                result.is_err(),
                "Should fail for invalid allocation: {}",
                address
            );
        }

        // Verify tree integrity maintained after invalid attempts
        assert_eq!(
            tree.root.value, initial_root,
            "Tree root should remain unchanged"
        );
    }
}
