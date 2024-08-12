use serde::Deserialize;
use starknet::core::types::Felt;
use starknet_crypto::{pedersen_hash};
use std::collections::HashSet;

#[derive(Deserialize, Debug, Clone)]
pub struct Allocation {
    pub address: String,
    pub amount: u64,
    pub timestamp: String,
}

pub struct MerkleTree {
    pub root: Node,
    allocations: Vec<Allocation>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_addresses: HashSet<Felt>,
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

    pub fn build_address_calldata(&self, address: &str, amount: u64, timestamp: &str) -> Result<Vec<String>, ()> {
        let felt_address = Felt::from_hex(address).map_err(|_| ())?;
        let felt_amount = u64_to_felt(amount);
        let felt_timestamp = Felt::from_hex(timestamp).map_err(|_| ())?;

        // Find the leaf node corresponding to the allocation
        if !self.root.accessible_addresses.contains(&felt_address) {
            return Err(());
        }

        let mut hashes: Vec<Felt> = vec![];
        let mut current_node = &self.root;
        loop {
            let left = current_node.left_child.as_ref().unwrap();
            let right = current_node.right_child.as_ref().unwrap();
            if left.accessible_addresses.contains(&felt_address) {
                hashes.push(right.value);
                current_node = left;
            } else {
                hashes.push(left.value);
                current_node = right;
            }
            if current_node.left_child.is_none() {
                break;
            }
        }
        hashes = hashes.into_iter().rev().collect();

        // Build the calldata
        let mut calldata = vec![felt_address, felt_amount, felt_timestamp];
        calldata.append(&mut hashes);

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
        let mut accessible_addresses = HashSet::new();
        accessible_addresses.extend(left_child.accessible_addresses.clone());
        accessible_addresses.extend(right_child.accessible_addresses.clone());

        Node {
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
            accessible_addresses,
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
            accessible_addresses: vec![address].into_iter().collect(),
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
            let right = if chunk.len() == 2 { chunk[1].clone() } else { left.clone() };
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