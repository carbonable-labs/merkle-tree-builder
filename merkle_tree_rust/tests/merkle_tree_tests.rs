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
#[should_panic(expected = "Invalid address")]
fn test_create_merkle_tree_with_invalid_allocation() {
    let allocations = vec![
        Allocation {
            address: "0x1234567dhiodhaoo".to_string(), // invalid
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

    MerkleTree::new(allocations);
}

#[test]
fn test_create_merkle_tree_with_odd_number_of_leaves() {
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
        Allocation {
            address: "0x892cdefabcdefabcdefabcdefabcdefabcdef1234".to_string(),
            amount: 200,
            timestamp: "0x3".to_string(),
            id: 2,
        },
    ];

    let tree = MerkleTree::new(allocations);
    assert!(!tree.root.value.is_zero());
    assert_eq!(
        tree.root.left_child.unwrap().accessible_allocations.len(),
        2
    );
    assert_eq!(
        tree.root.right_child.unwrap().accessible_allocations.len(),
        1
    );
}

#[test]
fn test_build_address_calldata() {
    let allocations = vec![Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    }];

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
#[should_panic(expected = "Failed to generate calldata")]
fn test_build_address_calldata_with_invalid_allocation() {
    let allocations = vec![Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    }];
    let unit_allocation = Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2josjojd".to_string(),
        id: 1,
    };

    let tree = MerkleTree::new(allocations.clone());
    let proof = tree
        .build_address_calldata(
            &unit_allocation.address,
            unit_allocation.amount,
            &unit_allocation.timestamp,
            unit_allocation.id,
        )
        .expect("Failed to generate calldata");
}

#[test]
fn test_merge_merkle_trees() {
    let allocations1 = vec![Allocation {
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
    }
    ];

    let allocations2 = vec![
        Allocation{
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: 150,
            timestamp: "0x2".to_string(),
            id: 1,
        }, // similar to previous allocation
        Allocation{
            address: "0x3F5A1E9DAB72F1A8C12D4D9B3A58A7B4425E7B4C".to_string(),
            amount: 250,
            timestamp: "0x4".to_string(),
            id: 5,
        }
    ];


    let tree1 = MerkleTree::new(allocations1);

    let merged_tree = tree1.merge_merkle_trees(allocations2.clone());
    assert!(!merged_tree.root.value.is_zero());
    assert_eq!(merged_tree.get_allocations().len(), 4);

    
}
