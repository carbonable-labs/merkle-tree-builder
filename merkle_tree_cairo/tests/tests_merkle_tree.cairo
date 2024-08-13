use core::result::ResultTrait;
// Starknet deps
use starknet::{ContractAddress, contract_address_const};

// External deps
use snforge_std as snf;
use snforge_std::{
    ContractClassTrait, test_address, spy_events, EventSpy, start_cheat_caller_address,
    stop_cheat_caller_address
};


// Contracts
use merkle_tree_cairo::claimer::{Claimer, IClaimerDispatcher, IClaimerDispatcherTrait};

// Constants
use super::constants::{HASH_ROOT_SET_1, get_data_simple_claim};

fn deploy_contract() -> ContractAddress {
    let contract = snf::declare("Claimer").expect('Declaration failed');

    let calldata: Array<felt252> = array![];
    let (contract_address, _) = contract.deploy(@calldata).expect('Deployment failed');

    contract_address
}

#[test]
fn test_set_merkle_root() {
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(HASH_ROOT_SET_1);

    let root = contract.get_merkle_root();
    assert_eq!(root, HASH_ROOT_SET_1);
}

#[test]
fn test_simple_claim() {
    // The merkle tree is the one of first_merkle_tree_data.txt, the allocation is the first one.
    // The user's allocation is in the merkle tree. It's the first and only allocation of the user. The user claims it.
    let (ROOT, user_address, amount, timestamp, proof) = get_data_simple_claim();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, user_address);
    contract.claim(amount, timestamp, proof);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, true);
}
