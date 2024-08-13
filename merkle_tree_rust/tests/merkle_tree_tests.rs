use merkle_tree_rust::{u64_to_felt, Allocation, MerkleTree};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_merkle_tree_creation() {
    let allocations = vec![
        Allocation {
            address: "0x123".to_string(),
            amount: 100,
            timestamp: "0x1".to_string(),
        },
        Allocation {
            address: "0x456".to_string(),
            amount: 200,
            timestamp: "0x2".to_string(),
        },
    ];
    let tree = MerkleTree::new(allocations);
    assert_eq!(tree.get_allocations().len(), 2);
}

#[test]
fn test_merkle_tree_proof() {
    let allocations = vec![
        Allocation {
            address: "0x123".to_string(),
            amount: 100,
            timestamp: "0x1".to_string(),
        },
        Allocation {
            address: "0x456".to_string(),
            amount: 200,
            timestamp: "0x2".to_string(),
        },
    ];
    let tree = MerkleTree::new(allocations.clone());
    let allocation = &allocations[0];
    let proof = tree.build_address_calldata(
        &allocation.address,
        allocation.amount,
        &allocation.timestamp,
    );
    assert!(proof.is_ok());

    // [Verification]
    let calldata = proof.unwrap();
    assert_eq!(calldata.len(), 3 + 1); // Address, amount, timestamp + 1 hash of proof
    assert_eq!(calldata[0], "0x123");
    assert_eq!(calldata[1], "0x64"); // 100 in hex is 0x64
    assert_eq!(calldata[2], "0x1");
}

#[test]
fn test_single_allocation() {
    // If odd number of allocations, the last one is duplicated
    let allocations = vec![Allocation {
        address: "0x123".to_string(),
        amount: 100,
        timestamp: "0x1".to_string(),
    }];
    let tree = MerkleTree::new(allocations.clone());
    assert_eq!(tree.get_allocations().len(), 1);

    let proof = tree.build_address_calldata("0x123", 100, "0x1");
    assert!(proof.is_ok());

    let calldata = proof.unwrap();
    assert_eq!(calldata.len(), 3 + 1);
    assert_eq!(calldata[0], "0x123");
    assert_eq!(calldata[1], "0x64"); // 100 in hex is 0x64
    assert_eq!(calldata[2], "0x1");
}

#[test]
fn test_large_amounts() {
    let allocations = vec![
        Allocation {
            address: "0xabc".to_string(),
            amount: 1_000_000_000_000, // Large amount
            timestamp: "0x1".to_string(),
        },
        Allocation {
            address: "0xdef".to_string(),
            amount: 2_000_000_000_000, // Large amount
            timestamp: "0x2".to_string(),
        },
    ];
    let tree = MerkleTree::new(allocations);
    assert_eq!(tree.get_allocations().len(), 2);

    let proof = tree.build_address_calldata("0xabc", 1_000_000_000_000, "0x1");
    assert!(proof.is_ok());

    let calldata = proof.unwrap();
    assert_eq!(calldata.len(), 3 + 1);
    assert_eq!(calldata[0], "0xabc");
    assert_eq!(calldata[1], "0xe8d4a51000"); // 1_000_000_000_000 in hex is 0xe8d4a51000
    assert_eq!(calldata[2], "0x1");
}

#[test]
fn test_invalid_address() {
    let allocations = vec![
        Allocation {
            address: "0x123".to_string(),
            amount: 100,
            timestamp: "0x1".to_string(),
        },
        Allocation {
            address: "0x456".to_string(),
            amount: 200,
            timestamp: "0x2".to_string(),
        },
    ];
    let tree = MerkleTree::new(allocations);

    let proof = tree.build_address_calldata("0x789", 100, "0x1");
    assert!(proof.is_err()); // should fail
}

#[test]
fn test_odd_number_of_allocations() {
    let allocations = vec![
        Allocation {
            address: "0x111".to_string(),
            amount: 50,
            timestamp: "0x1".to_string(),
        },
        Allocation {
            address: "0x222".to_string(),
            amount: 75,
            timestamp: "0x2".to_string(),
        },
        Allocation {
            address: "0x333".to_string(),
            amount: 125,
            timestamp: "0x3".to_string(),
        },
    ];
    let tree = MerkleTree::new(allocations.clone());
    assert_eq!(tree.get_allocations().len(), 3);

    let allocation = &allocations[0];
    let proof = tree.build_address_calldata(
        &allocation.address,
        allocation.amount,
        &allocation.timestamp,
    );
    assert!(proof.is_ok());

    let calldata = proof.unwrap();
    assert_eq!(calldata.len(), 3 + 2); // Address, amount, timestamp + 2 hashes of proof
    assert_eq!(calldata[0], "0x111");
    assert_eq!(calldata[1], "0x32"); // 50 in hex is 0x32
    assert_eq!(calldata[2], "0x1");
}

#[test]
fn test_merkle_tree_with_mock_data() {
    let file = File::open("tests/mock_allocations_first_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let allocations: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    let tree = MerkleTree::new(allocations.clone());

    assert_eq!(tree.get_allocations().len(), allocations.len());
    assert!(format!("{:#x}", tree.root.value).starts_with("0x"));

    // Verify proof generation for each allocation
    for allocation in allocations.iter() {
        let proof = tree.build_address_calldata(
            &allocation.address,
            allocation.amount,
            &allocation.timestamp,
        );
        assert!(
            proof.is_ok(),
            "Proof generation failed for allocation {:?}",
            allocation.address
        );

        let calldata = proof.unwrap();
        println!("Calldata for {}: {:?}", allocation.address, calldata);

        // Verify that the right amount is included in the proof
        assert!(
            calldata.contains(&format!("{:#x}", u64_to_felt(allocation.amount))),
            "The amount {:#x} is not included in the proof for address {}",
            allocation.amount,
            allocation.address
        );
    }
}

#[test]
fn test_merge_trees_with_new_allocations() {
    // Load the old Merkle tree
    let file = File::open("tests/mock_allocations_first_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let old_allocations: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    let old_tree = MerkleTree::new(old_allocations.clone());

    // Load the new allocations from the JSON file
    let file = File::open("tests/mock_allocations_second_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let new_allocations: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    // Merge the trees
    let merged_tree = old_tree.merge_merkle_trees(new_allocations.clone());

    assert!(merged_tree.get_allocations().len() > old_tree.get_allocations().len());
    println!("Merged Root hash: {:?}", merged_tree.root.value);
    assert!(format!("{:#x}", merged_tree.root.value).starts_with("0x"));

    // Verify proof generation for each allocation from the old tree
    for allocation in old_allocations.iter() {
        let proof = merged_tree.build_address_calldata(
            &allocation.address,
            allocation.amount,
            &allocation.timestamp,
        );
        assert!(
            proof.is_ok(),
            "Proof generation failed for old allocation {:?}",
            allocation.address
        );

        let calldata = proof.unwrap();
        assert!(
            calldata.contains(&format!("{:#x}", u64_to_felt(allocation.amount))),
            "The amount {:#x} is not included in the proof for address {}",
            allocation.amount,
            allocation.address
        );
    }

    // Verify proof generation for each allocation from the new tree
    for allocation in new_allocations.iter() {
        let proof = merged_tree.build_address_calldata(
            &allocation.address,
            allocation.amount,
            &allocation.timestamp,
        );
        assert!(
            proof.is_ok(),
            "Proof generation failed for new allocation {:?}",
            allocation.address
        );

        let calldata = proof.unwrap();
        assert!(
            calldata.contains(&format!("{:#x}", u64_to_felt(allocation.amount))),
            "The amount {:#x} is not included in the proof for address {}",
            allocation.amount,
            allocation.address
        );
    }
}
