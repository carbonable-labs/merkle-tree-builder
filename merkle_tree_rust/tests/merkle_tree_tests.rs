use merkle_tree::core::allocation::Allocation;
use merkle_tree::core::merkle_tree::MerkleTree;
use num_traits::Zero;

#[test]
fn test_create_merkle_tree() {
    let allocations = vec![
        Allocation {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: 150,
            timestamp: "0x2".to_string(),
            id: 1,
        },
        Allocation {
            address: "0xabcdefabcdefabcdefabcdefabcdefabcdef1234".to_string(),
            amount: 200,
            timestamp: "0x3".to_string(),
            id: 2,
        },
    ];

    let tree = MerkleTree::new(allocations);
    assert!(!tree.root.value.is_zero()); // `is_zero` now works
}

#[test]
fn test_build_address_calldata() {
    let allocations = vec![
        Allocation {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: 150,
            timestamp: "0x2".to_string(),
            id: 1,
        },
    ];

    let tree = MerkleTree::new(allocations.clone());
    let proof = tree
        .build_address_calldata(
            &allocations[0].address,
            allocations[0].amount,
            &allocations[0].timestamp,
            allocations[0].id,
        )
        .expect("Failed to generate calldata");

    assert!(!proof.is_empty());
}

#[test]
fn test_merge_merkle_trees() {
    let allocations1 = vec![Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    }];

    let allocations2 = vec![Allocation {
        address: "0xabcdefabcdefabcdefabcdefabcdefabcdef1234".to_string(),
        amount: 200,
        timestamp: "0x3".to_string(),
        id: 2,
    }];

    let tree1 = MerkleTree::new(allocations1);
    let tree2 = MerkleTree::new(allocations2);

    let merged_tree = tree1.merge_merkle_trees(tree2.get_allocations().clone());
    assert!(!merged_tree.root.value.is_zero());
    assert_eq!(merged_tree.get_allocations().len(), 2);
}
