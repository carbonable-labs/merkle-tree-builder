use starknet::{ContractAddress};

#[starknet::interface]
pub trait IClaimer<TContractState> {
    fn claim(ref self: TContractState, amount: u128, timestamp: u128, proof: Array::<felt252>);

    fn check_claimed(
        ref self: TContractState, claimee: ContractAddress, timestamp: u128, amount: u128
    ) -> bool;

    fn set_merkle_root(ref self: TContractState, root: felt252);

    fn get_merkle_root(ref self: TContractState) -> felt252;
}

#[starknet::contract]
pub mod Claimer {
    use starknet::{ContractAddress, ClassHash, get_caller_address};
    use alexandria_merkle_tree::merkle_tree::{
        Hasher, MerkleTree, MerkleTreeImpl, pedersen::PedersenHasherImpl, MerkleTreeTrait,
    };
    use core::hash::LegacyHash;

    #[derive(Copy, Drop, Debug, Hash, starknet::Store, Serde, PartialEq)]
    struct Allocation {
        claimee: ContractAddress,
        amount: u128,
        timestamp: u128
    }

    #[storage]
    struct Storage {
        merkle_root: felt252,
        // Mapping from .
        allocations_claimed: LegacyMap<
            Allocation, bool
        >, // todo: several deposit for same timestamp may cause issues
    }


    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        Claimed: Claimed,
    }

    #[derive(Drop, starknet::Event)]
    pub struct Claimed {
        pub claimee: ContractAddress,
        pub amount: u128,
        pub timestamp: u128
    }

    // Externals
    #[abi(embed_v0)]
    impl ClaimerImpl of super::IClaimer<ContractState> {
        fn claim(ref self: ContractState, amount: u128, timestamp: u128, proof: Array::<felt252>) {
            let mut merkle_tree: MerkleTree<Hasher> = MerkleTreeImpl::new();
            let claimee = get_caller_address();
            // [Verify the proof]
            let amount_felt: felt252 = amount.into();
            let claimee_felt: felt252 = claimee.into();
            let timestamp_felt: felt252 = timestamp.into();

            let intermediate_hash = LegacyHash::hash(claimee_felt, amount_felt);
            let leaf = LegacyHash::hash(intermediate_hash, timestamp_felt);

            let root_computed = merkle_tree.compute_root(leaf, proof.span());

            let stored_root = self.merkle_root.read();
            assert(root_computed == stored_root, 'Invalid proof');

            // [Verify not already claimed]
            let claimed = self.check_claimed(claimee, timestamp, amount);
            assert(!claimed, 'Already claimed');

            // [Mark as claimed]
            let allocation = Allocation { claimee: claimee, amount: amount, timestamp: timestamp };
            self.allocations_claimed.write(allocation, true);

            // [Emit event]
            self.emit(Claimed { claimee: claimee, amount: amount, timestamp: timestamp });
        }

        fn check_claimed(
            ref self: ContractState, claimee: ContractAddress, timestamp: u128, amount: u128
        ) -> bool {
            // check if claimee has already claimed for this timestamp, by checking in the mapping
            let allocation = Allocation {
                claimee: claimee, amount: amount.into(), timestamp: timestamp.into()
            };
            self.allocations_claimed.read(allocation)
        }

        fn set_merkle_root(ref self: ContractState, root: felt252) {
            self.merkle_root.write(root);
        }

        fn get_merkle_root(ref self: ContractState) -> felt252 {
            self.merkle_root.read()
        }
    }
}
