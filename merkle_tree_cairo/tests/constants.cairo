use starknet::{ContractAddress, contract_address_const};

pub const MERKLE_ROOT_FIRST_WAVE: felt252 =
    803781063426407299979325390167664109772842041387232186868510660774343066272;

pub const MERKLE_ROOT_SECOND_WAVE: felt252 =
    3023878233865233747692111000084174893656568287435392306059398425498163029420;

pub fn get_bob_first_wave_allocation() -> (
    felt252, ContractAddress, u128, u128, u128, Array<felt252>
) {
    let address: ContractAddress = contract_address_const::<
        0x1234567890abcdef1234567890abcdef12345678
    >();
    let amount: u128 = 150;
    let timestamp: u128 = 2;
    let id: u128 = 1;

    let proof: Array<felt252> = array![
        0x2fc0d4eecd4e047701f1a8295209d8a4d2b243836f5cf78df91bd073ce49084,
        0x5fed9820061cf127fb1689269a6d53d72c3d1f289aff4bac0afea2103b5f229,
        0x6ef44033073498cfd5dc97338ffe3afd139a87d56c1045ccefc3108a653b6f2,
        0x6b04f0ca9a85505cd6cae37c678dd899f200b92639474e6e594fcf02544ed42,
        0x1855303a4c287845b59acbe58e85df3618e6e3dbc27ffb7554e565ec3a606b0
    ];

    (MERKLE_ROOT_FIRST_WAVE, address, amount, timestamp, id, proof)
}

pub fn get_bob_second_wave_allocation() -> (
    felt252, ContractAddress, u128, u128, u128, Array<felt252>
) {
    let address: ContractAddress = contract_address_const::<
        0x1234567890abcdef1234567890abcdef12345678
    >();
    let amount: u128 = 150;
    let timestamp: u128 = 2;
    let id: u128 = 1;

    let proof: Array<felt252> = array![
        0x2fc0d4eecd4e047701f1a8295209d8a4d2b243836f5cf78df91bd073ce49084,
        0x5fed9820061cf127fb1689269a6d53d72c3d1f289aff4bac0afea2103b5f229,
        0x6ef44033073498cfd5dc97338ffe3afd139a87d56c1045ccefc3108a653b6f2,
        0x6b04f0ca9a85505cd6cae37c678dd899f200b92639474e6e594fcf02544ed42,
        0x1855303a4c287845b59acbe58e85df3618e6e3dbc27ffb7554e565ec3a606b0,
        0x545687bbf6429d9a0664d6892ce9fc45b98f9529229358e252302434d85976c
    ];

    (MERKLE_ROOT_SECOND_WAVE, address, amount, timestamp, id, proof)
}

pub fn get_alice_second_wave_allocation() -> (
    felt252, ContractAddress, u128, u128, u128, Array<felt252>
) {
    let address: ContractAddress = contract_address_const::<
        0xabcdefabcdefabcdefabcdefabcdefabcdefabc
    >();
    let amount: u128 = 800;
    let timestamp: u128 = 13;
    let id: u128 = 1;

    let proof: Array<felt252> = array![
        0x387e71c3fe5c7ed5e81814e57bbdd88c9cc249b9071d626a8669bb8e6fb38bc,
        0x3c1fa52fc063ceea9fdf3790b3d4b86698c7239ca857226957fee50f0ebc01d,
        0x13e92543d838d5c721017891f665092a2b5558f47ac544e5c0a3867c6ba5cbf,
        0x3b69ab80dcf08d633999db77d659e3bb7cb79270a1db1fdf5c432a950375cf7,
        0x45c532062ad92e4bf5e4fc2b755c6cca48b03ae8c89b7eba239a21a3253ac4f,
        0x1c6ec88a48638cc8c14e1c72767d58860a86cefbdd696d24e1253c0f6c1c2a0
    ];

    (MERKLE_ROOT_SECOND_WAVE, address, amount, timestamp, id, proof)
}

pub fn get_john_multiple_allocations() -> (
    felt252,
    felt252,
    ContractAddress,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    u128,
    Array<felt252>,
    Array<felt252>,
    Array<felt252>,
    Array<felt252>
) {
    let address: ContractAddress = contract_address_const::<
        0xabcdefabcdef1234567890abcdef1234567890ab
    >();

    // Allocation 1
    let amount1: u128 = 700;
    let timestamp1: u128 = 0x6;
    let id_1: u128 = 1;
    let proof1: Array<felt252> = array![
        0x6ac1aae7e68c4e203c00d8eff310bbca90f90ae3badaa8b6f6bf637ee52eec,
        0x2c91a9511ef588d90f7f89f513595c75bc24ea19e18c0bb740dcda20027ca56,
        0x431297a4c5039b6198b4ea942e06c480aa662334f252fb2941c537f458c4ca8,
        0x6b04f0ca9a85505cd6cae37c678dd899f200b92639474e6e594fcf02544ed42,
        0x1855303a4c287845b59acbe58e85df3618e6e3dbc27ffb7554e565ec3a606b0
    ];

    // Allocation 2
    let amount2: u128 = 900;
    let timestamp2: u128 = 17;
    let id_2: u128 = 2;
    let proof2: Array<felt252> = array![
        0x2271d27a5469a12d5854af8d6dd19924b4ce389b347bad9660714d65d5ea849,
        0x2d4f077932acdce076172e418dedd99d369ab390e0ecaa4441346027b280287,
        0x11536a6a75883757f0e46fe84a6c0550c1d72f3a6e827e86c72a86bc200d73a,
        0x2e996dca1817edb8d42d2312b9dbc9ff2f79d5ec3c029b6fe3937f8ded5d01d,
        0x1855303a4c287845b59acbe58e85df3618e6e3dbc27ffb7554e565ec3a606b0
    ];

    // Allocation 3
    let amount3: u128 = 2500;
    let timestamp3: u128 = 0x18;
    let id_3: u128 = 3;
    let proof3: Array<felt252> = array![
        0x243eb22d79b86e04e2665bac9cf3a42465edba7bb8fe1630a821c4593ca781a,
        0x26a185f92c71cf586a662182d4f5dd5ac2812be84e44a0d463bd411b2c5805e,
        0x629b8d38174754785a8d32fee5d790a9aa644df167fc83263888fd70835295,
        0x4d2752b3411df566e417454f8533c2a8a21f61bf6e705d33b6dc3d903c91ca2,
        0x61bdd78c2e4b89f38ef7492670e4744a0885b7c776ffb254d1c9b73c850fdf5
    ];

    // Allocation 4 of the second wave
    let amount4: u128 = 287;
    let timestamp4: u128 = 0xE;
    let id_4: u128 = 4;
    let proof4: Array<felt252> = array![
        0x49118c782a2a6c1ceb9890535f1d2fcca16a8b1d916ca1af4f8eadb7f8b8e0a,
        0x74c176d79348e16a11489735a3fb593c6bc855abed8efbfaa81a29fd9e0a893,
        0x13e92543d838d5c721017891f665092a2b5558f47ac544e5c0a3867c6ba5cbf,
        0x3b69ab80dcf08d633999db77d659e3bb7cb79270a1db1fdf5c432a950375cf7,
        0x45c532062ad92e4bf5e4fc2b755c6cca48b03ae8c89b7eba239a21a3253ac4f,
        0x1c6ec88a48638cc8c14e1c72767d58860a86cefbdd696d24e1253c0f6c1c2a0
    ];

    (
        MERKLE_ROOT_FIRST_WAVE,
        MERKLE_ROOT_SECOND_WAVE,
        address,
        amount1,
        timestamp1,
        id_1,
        amount2,
        timestamp2,
        id_2,
        amount3,
        timestamp3,
        id_3,
        amount4,
        timestamp4,
        id_4,
        proof1,
        proof2,
        proof3,
        proof4
    )
}
