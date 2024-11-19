use crate::allocation::Allocation;
use crate::node::Node;
use starknet::core::types::Felt;

pub struct MerkleTree {
    pub root: Node,
    allocations: Vec<Allocation>,
}

impl MerkleTree {
    /// Creates a new Merkle tree from a list of allocations.
    pub fn new(allocations: Vec<Allocation>) -> Self {
        let mut leaves: Vec<Node> = allocations
            .clone()
            .into_iter()
            .map(Node::new_leaf)
            .collect();

        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let root = build_tree(leaves);

        MerkleTree { root, allocations }
    }

    /// Returns the list of allocations in the Merkle tree.
    pub fn get_allocations(&self) -> &Vec<Allocation> {
        &self.allocations
    }

    /// Generates calldata for a specific allocation.
    pub fn build_address_calldata(
        &self,
        address: &str,
        amount: u64,
        timestamp: &str,
        id: u64,
    ) -> Result<Vec<String>, String> {
        let allocation = Allocation {
            address: address.to_string(),
            amount,
            timestamp: timestamp.to_string(),
            id,
        };

        // Traverse the tree to find the proof path
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
                return Err("Allocation not found".to_string());
            }
        }

        hashes.reverse();
        let mut calldata = allocation.to_felts().unwrap().to_vec();
        calldata.extend(hashes);

        Ok(calldata.iter().map(|f| format!("{:#x}", f)).collect())
    }

    /// Merges the current tree with new allocations.
    pub fn merge_merkle_trees(&self, new_allocations: Vec<Allocation>) -> MerkleTree {
        let mut combined_allocations = self.get_allocations().clone();
        combined_allocations.extend(new_allocations.clone());
        MerkleTree::new(combined_allocations)
    }
}

/// Builds the Merkle tree from leaf nodes.
fn build_tree(mut leaves: Vec<Node>) -> Node {
    while leaves.len() > 1 {
        let mut next_level = vec![];
        for chunk in leaves.chunks(2) {
            let left = chunk[0].clone();
            let right = if chunk.len() == 2 { chunk[1].clone() } else { left.clone() };
            next_level.push(Node::new(left, right));
        }
        leaves = next_level;
    }
    leaves.remove(0)
}
