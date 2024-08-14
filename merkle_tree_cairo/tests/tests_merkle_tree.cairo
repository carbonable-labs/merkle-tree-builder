use snforge_std::cheatcodes::events::EventSpyAssertionsTrait;
use core::clone::Clone;
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
use super::constants::{
    HASH_ROOT_SET_1, get_data_simple_claim_bob_alloc, get_combined_data_bob_alloc,
    get_combined_data_alice_alloc, get_allocs_first_wave_john, get_data_double_claim_bob_alloc
};

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
    let (ROOT, user_address, amount, timestamp, proof) = get_data_simple_claim_bob_alloc();
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

#[test]
fn test_double_claim() {
    // The user got 2 allocations with different amounts and timestamps. He claims them both.
    let ROOT = 0x7be624496a1672132d43607a5aa283940fe5ea7bbdee39b34606ea895a91e05;
    let address = contract_address_const::<0x123>();
    let amount1 = 0x64;
    let timestamp1 = 0x1;
    let proof1 = array![
        0x7eca5c09f03332447da957fea8123007825b06a57286c4fcfc35f130d450588,
        0x5e5ec048d441d324737e44bde3bb146ad3f782fba4ebabd52017f67f381969b
    ];

    let amount2 = 0x6e;
    let timestamp2 = 0x13;
    let proof2 = array![
        0x6a716202ba08ca9b9f199b0d54d80508fd7a30687f821ec13478a166f1eaec0,
        0xd31a5d1a36d3a6c35438017cacb878814c47b7fb49245ce812323e96790a87
    ];

    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);

    let claimed1 = contract.check_claimed(address, timestamp1, amount1);
    assert_eq!(claimed1, false);

    start_cheat_caller_address(contract_address, address);

    contract.claim(amount1, timestamp1, proof1);

    let claimed1 = contract.check_claimed(address, timestamp1, amount1);
    assert_eq!(claimed1, true);

    let claimed2 = contract.check_claimed(address, timestamp2, amount2);
    assert_eq!(claimed2, false);

    contract.claim(amount2, timestamp2, proof2);
    let claimed2 = contract.check_claimed(address, timestamp2, amount2);
    assert_eq!(claimed2, true);
}

#[test]
#[should_panic(expected: 'Already claimed')]
fn test_panic_simple_claim_twice() {
    // The merkle tree is the one of first_merkle_tree_data.txt, the allocation is the first one.
    // The user's allocation is in the merkle tree. It's the first and only allocation of the user. The user claims it.
    let (ROOT, user_address, amount, timestamp, proof) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, user_address);
    contract.claim(amount, timestamp, proof.clone());

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, true);

    contract.claim(amount, timestamp, proof);
}

#[test]
#[should_panic(expected: 'Invalid proof')]
fn test_panic_simple_claim_invalid_leaf_address() {
    let (ROOT, _, amount, timestamp, proof) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let invalid_user_address = contract_address_const::<'DUMMY'>();
    let claimed = contract.check_claimed(invalid_user_address, timestamp, amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, invalid_user_address);
    contract.claim(amount, timestamp, proof);
}

#[test]
#[should_panic(expected: 'Invalid proof')]
fn test_panic_simple_claim_invalid_leaf_amount() {
    let (ROOT, user_address, _, timestamp, proof) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let dummy_amount = 0;

    let claimed = contract.check_claimed(user_address, timestamp, dummy_amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, user_address);
    contract.claim(dummy_amount, timestamp, proof);
}

#[test]
#[should_panic(expected: 'Invalid proof')]
fn test_panic_simple_claim_invalid_leaf_timestamp() {
    let (ROOT, user_address, amount, _, proof) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let dummy_timestamp = 0;

    let claimed = contract.check_claimed(user_address, dummy_timestamp, amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, user_address);
    contract.claim(amount, dummy_timestamp, proof);
}

#[test]
#[should_panic(expected: 'Invalid proof')]
fn test_panic_simple_claim_invalid_leaf_proof() {
    let (ROOT, user_address, amount, timestamp, _) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let dummy_proof: Array<felt252> = array![0x123, 0x1];

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, user_address);
    contract.claim(amount, timestamp, dummy_proof);
}

#[test]
fn test_emit_event_simple_claim() {
    // The merkle tree is the one of first_merkle_tree_data.txt, the allocation is the first one.
    // The user's allocation is in the merkle tree. It's the first and only allocation of the user. The user claims it.
    let (ROOT, user_address, amount, timestamp, proof) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, false);

    let mut spy = spy_events();
    start_cheat_caller_address(contract_address, user_address);
    contract.claim(amount, timestamp, proof);

    let expected_event = Claimer::Event::Claimed(
        Claimer::Claimed { claimee: user_address, amount: amount, timestamp: timestamp }
    );

    spy.assert_emitted(@array![(contract_address, expected_event)]);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, true);
}

#[test]
fn test_combined_root_still_claimable() {
    // User can claim on wave 1. He doesn't. Wave 2 is set with a new hash root. User can still claim his wave 1 allocation.
    let (ROOT, user_address, amount, timestamp, _) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, false);

    let (NEW_ROOT, _, _, _, new_proof) = get_combined_data_bob_alloc();
    contract.set_merkle_root(NEW_ROOT);

    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, false);

    start_cheat_caller_address(contract_address, user_address);
    contract.claim(amount, timestamp, new_proof);
    let claimed = contract.check_claimed(user_address, timestamp, amount);
    assert_eq!(claimed, true);
}

#[test]
fn test_claim_second_wave() {
    // Alice had no allocation in wave 1. Bob did. Bob claimed her allocation in wave 1. Alice has an allocation in wave 2. Alice claims his allocation in wave 2.
    let (ROOT, bob_address, amount, timestamp, proof) = get_data_simple_claim_bob_alloc();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    let bob_claimed = contract.check_claimed(bob_address, timestamp, amount);
    assert_eq!(bob_claimed, false);

    let mut spy = spy_events();
    start_cheat_caller_address(contract_address, bob_address);
    contract.claim(amount, timestamp, proof);
    let expected_event = Claimer::Event::Claimed(
        Claimer::Claimed { claimee: bob_address, amount: amount, timestamp: timestamp }
    );
    spy.assert_emitted(@array![(contract_address, expected_event)]);
    let bob_claimed = contract.check_claimed(bob_address, timestamp, amount);
    assert_eq!(bob_claimed, true);

    let (NEW_ROOT, alice_address, amount, timestamp, proof) = get_combined_data_alice_alloc();
    let alice_claimed = contract.check_claimed(alice_address, timestamp, amount);
    assert_eq!(alice_claimed, false);

    contract.set_merkle_root(NEW_ROOT);
    start_cheat_caller_address(contract_address, alice_address);
    contract.claim(amount, timestamp, proof);

    let alice_claimed = contract.check_claimed(alice_address, timestamp, amount);
    assert_eq!(alice_claimed, true);
}

#[test]
fn test_claim_first_john() {
    // John has 3 allocations in wave 1. He claims 2 of them. Then we set wave 2 where he gets 1 new allocation. He claims it, but not the 3rd allocation of wave 1.
    let (
        ROOT,
        NEW_ROOT,
        john_address,
        amount1,
        timestamp1,
        amount2,
        timestamp2,
        amount3,
        timestamp3,
        amount4,
        timestamp4,
        proof1,
        proof2,
        _,
        proof4
    ) =
        get_allocs_first_wave_john();
    let contract_address = deploy_contract();

    let contract = IClaimerDispatcher { contract_address: contract_address };
    contract.set_merkle_root(ROOT);
    let root = contract.get_merkle_root();
    assert_eq!(root, ROOT);

    assert!(!contract.check_claimed(john_address, timestamp1, amount1));
    assert!(!contract.check_claimed(john_address, timestamp2, amount2));
    assert!(!contract.check_claimed(john_address, timestamp3, amount3));
    start_cheat_caller_address(contract_address, john_address);
    contract.claim(amount1, timestamp1, proof1);
    contract.claim(amount2, timestamp2, proof2);
    assert!(contract.check_claimed(john_address, timestamp1, amount1));
    assert!(contract.check_claimed(john_address, timestamp2, amount2));
    assert!(!contract.check_claimed(john_address, timestamp3, amount3));

    contract.set_merkle_root(NEW_ROOT);

    contract.claim(amount4, timestamp4, proof4);
    assert!(contract.check_claimed(john_address, timestamp4, amount4));
    assert!(!contract.check_claimed(john_address, timestamp3, amount3));
}
