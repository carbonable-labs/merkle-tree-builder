use starknet::{ContractAddress, contract_address_const};

pub const HASH_ROOT_SET_1: felt252 =0x2821297ca54382d6e522b5f7aaf7a18d8e4cc7b490abea3e5499c88ff6171f6;

pub fn get_data_simple_claim() -> (felt252, ContractAddress, u128, u128, Array<felt252>) {
    let address: ContractAddress = contract_address_const::<0x1234567890abcdef1234567890abcdef12345678>();
    let amount: u128 = 150;
    let timestamp: u128 = 2;

    let proof: Array<felt252> = array![0xb93b7d65a7e5c7a15def73a61485111e2f630cc8e6683fb98f4d6ca2c7ec96,0x91ca2d84afc873630898de633b3041683eea2a5d1d59ae4f3bed3551bb4294, 0x61c8f928bb4f7b5a3ae252cc4c78f3bdb3442733951a1de12ec01e7a4812a50, 0x6f0149c2ccc9a95bb64deda90572a912f85139505ea9bcd233f6d16e751af9e, 0x79b090d0858c9816b6e2c4a088d99231b3fc88188af9364998a02df25b7e4d8];

    return (HASH_ROOT_SET_1, address, amount, timestamp, proof);
}