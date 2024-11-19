use merkle_tree::core::allocation::Allocation;
use merkle_tree::core::node::Node;

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
