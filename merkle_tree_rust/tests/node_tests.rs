use merkle_tree::core::allocation::Allocation;
use merkle_tree::core::node::Node;
use starknet_crypto::pedersen_hash;
use std::any::{Any, TypeId};

#[test]
fn test_create_leaf_node() {
    let allocation = Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    };

    let node = Node::new_leaf(allocation.clone());
    assert!(node.left_child.is_none());
    assert!(node.right_child.is_none());
    assert_eq!(node.accessible_allocations.len(), 1);
    assert!(node.accessible_allocations.contains(&allocation));
}

#[test]
fn test_combine_nodes() {
    let allocation1 = Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    };

    let allocation2 = Allocation {
        address: "0xabcdefabcdefabcdefabcdefabcdefabcdef1234".to_string(),
        amount: 200,
        timestamp: "0x3".to_string(),
        id: 2,
    };

    let node1 = Node::new_leaf(allocation1);
    let node2 = Node::new_leaf(allocation2);

    let parent = Node::new(node1, node2);
    assert!(parent.left_child.is_some());
    assert!(parent.right_child.is_some());
    assert_eq!(parent.accessible_allocations.len(), 2);
}

#[test]
fn test_create_leaf_node_with_valid_allocation() {
    let allocation = Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    };
    let node = Node::new_leaf(allocation.clone());
    // check whether node is instance of Node struct
    assert_eq!(node.type_id(), TypeId::of::<Node>());
    assert_eq!(node.accessible_allocations.len(), 1);
    assert!(node.accessible_allocations.contains(&allocation));
    let (address, amount, timestamp, id) = allocation.to_felts().unwrap();
    let value_hash = pedersen_hash(&address, &amount);
    let value_hash = pedersen_hash(&value_hash, &timestamp);
    let value_hash = pedersen_hash(&value_hash, &id);
    assert_eq!(node.value, value_hash);
}

#[test]
#[should_panic(expected = "Invalid address")]
fn test_create_leaf_node_with_invalid_allocation() {
    let allocation = Allocation {
        address: "0x1234bcdef1uhds8".to_string(),
        amount: 150,
        timestamp: "0x20dsom".to_string(),
        id: 1,
    };
    let node = Node::new_leaf(allocation.clone());
    assert_ne!(node.type_id(), TypeId::of::<Node>());
}

#[test]
fn test_combine_identical_nodes() {
    let allocation = Allocation {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        amount: 150,
        timestamp: "0x2".to_string(),
        id: 1,
    };
    let node = Node::new_leaf(allocation.clone());
    let node2 = Node::new_leaf(allocation.clone());
    let parent_node = Node::new(node, node2);
    assert_eq!(parent_node.accessible_allocations.len(), 1);
    assert!(parent_node.left_child.is_some());
    assert!(parent_node.right_child.is_some());
}
