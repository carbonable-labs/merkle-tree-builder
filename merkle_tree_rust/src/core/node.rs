use crate::core::allocation::{Allocation};
use starknet::core::types::Felt;
use starknet_crypto::pedersen_hash;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_allocations: HashSet<Allocation>,
    pub value: Felt,
}

impl Node {
    /// Combines two nodes into a new parent node.
    pub fn new(a: Node, b: Node) -> Self {
        let (left_child, right_child) = if a.value < b.value { (a, b) } else { (b, a) };
        let value = pedersen_hash(&left_child.value, &right_child.value);
        let mut accessible_allocations = HashSet::new();
        accessible_allocations.extend(left_child.accessible_allocations.clone());
        accessible_allocations.extend(right_child.accessible_allocations.clone());

        Node {
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
            accessible_allocations,
            value,
        }
    }

    /// Creates a new leaf node from an allocation.
    pub fn new_leaf(allocation: Allocation) -> Self {
        let (address, amount, timestamp, id) = allocation.to_felts().unwrap();

        let intermediate_hash = pedersen_hash(&address, &amount);
        let intermediate_hash = pedersen_hash(&intermediate_hash, &timestamp);
        let value = pedersen_hash(&intermediate_hash, &id);

        Node {
            left_child: None,
            right_child: None,
            accessible_allocations: vec![allocation].into_iter().collect(),
            value,
        }
    }
}
