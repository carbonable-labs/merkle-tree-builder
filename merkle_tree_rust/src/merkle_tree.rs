use serde::Deserialize;
use starknet::core::types::Felt;
use starknet_crypto::pedersen_hash;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]pub struct Allocation {
    pub address: String,
    pub amount: u64,
    pub timestamp: String,
}

impl Hash for Allocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
        self.amount.hash(state);
        self.timestamp.hash(state);
    }
}

pub struct MerkleTree { 
    pub root: Node,
    allocations: Vec<Allocation>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_allocations: HashSet<Allocation>,
    pub value: Felt,
}

impl MerkleTree {
    pub fn new(allocations: Vec<Allocation>) -> Self {
        let mut leaves: Vec<Node> = allocations
            .clone()
            .into_iter()
            .map(|a| Node::new_leaf(a))
            .collect();

        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let root = build_tree(leaves);

        MerkleTree { root, allocations }
    }

    pub fn get_allocations(&self) -> &Vec<Allocation> {
        &self.allocations
    }

    pub fn build_address_calldata(
        &self,
        address: &str,
        amount: u64,
        timestamp: &str,
    ) -> Result<Vec<String>, ()> {
        let allocation = Allocation {
            address: address.to_string(),
            amount,
            timestamp: timestamp.to_string(),
        };
    
        // Compute the leaf node hash for this allocation
        let felt_address = Felt::from_hex(address).map_err(|_| ())?;
        let felt_amount = u64_to_felt(amount);
        let felt_timestamp = Felt::from_hex(timestamp).map_err(|_| ())?;
    
        let intermediate_hash = pedersen_hash(&felt_address, &felt_amount);
        let target_leaf = pedersen_hash(&intermediate_hash, &felt_timestamp);
    
        // Traverse the tree to find the proof path for the target leaf
        let mut hashes: Vec<Felt> = vec![];
        let mut current_node = &self.root;
    
        loop {
            if current_node.left_child.is_none() && current_node.right_child.is_none() {
                break;
            }
    
            let left = current_node.left_child.as_ref().unwrap();
            let right = current_node.right_child.as_ref().unwrap();
    
            if left.accessible_allocations.contains(&allocation) {
                hashes.push(right.value);
                current_node = left;
            } else if right.accessible_allocations.contains(&allocation) {
                hashes.push(left.value);
                current_node = right;
            } else {
                return Err(()); // Allocation not found
            }
        }
    
        hashes.reverse();
        let mut calldata = vec![felt_address, felt_amount, felt_timestamp];
        calldata.extend(hashes);
    
        Ok(calldata.iter().map(|f| format!("{:#x}", f)).collect())
    }
    

    pub fn merge_merkle_trees(&self, new_allocations: Vec<Allocation>) -> MerkleTree {
        let mut combined_allocations = self.get_allocations().clone();
        combined_allocations.extend(new_allocations.clone());
        MerkleTree::new(combined_allocations)
    }
}

impl Node {
    fn new(a: Node, b: Node) -> Self {
        let (left_child, right_child) = match a.value.lt(&b.value) {
            true => (a, b),
            false => (b, a),
        };
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

    fn new_leaf(allocation: Allocation) -> Self {
        let address = Felt::from_hex(&allocation.address).unwrap();
        let amount = u64_to_felt(allocation.amount);
        let timestamp = Felt::from_hex(&allocation.timestamp).unwrap();

        let intermediate_hash = pedersen_hash(&address, &amount);
        let value = pedersen_hash(&intermediate_hash, &timestamp);

        Node {
            left_child: None,
            right_child: None,
            accessible_allocations: vec![allocation].into_iter().collect(),
            value,
        }
    }
}

fn build_tree(leaves: Vec<Node>) -> Node {
    let mut nodes = leaves;
    while nodes.len() > 1 {
        let mut next_level = vec![];
        for chunk in nodes.chunks(2) {
            let left = chunk[0].clone();
            let right = if chunk.len() == 2 {
                chunk[1].clone()
            } else {
                left.clone()
            };
            next_level.push(Node::new(left, right));
        }
        nodes = next_level;
    }
    nodes.remove(0)
}

pub fn u64_to_felt(value: u64) -> Felt {
    let mut bytes = [0u8; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    Felt::from_bytes_be(&bytes)
}
