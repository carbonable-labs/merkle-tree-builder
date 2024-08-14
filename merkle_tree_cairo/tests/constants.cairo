use starknet::{ContractAddress, contract_address_const};

pub const HASH_ROOT_SET_1: felt252 =
    1586727653310658130441223142145636802822549738865763467559937699735593529518;

pub const HASH_ROOT_SET_2: felt252 =
    1254903502166521005693176785783698867375816882648399305753662436577997689730;

pub fn get_data_simple_claim_bob_alloc() -> (felt252, ContractAddress, u128, u128, Array<felt252>) {
    let address: ContractAddress = contract_address_const::<
        0x1234567890abcdef1234567890abcdef12345678
    >();
    let amount: u128 = 150;
    let timestamp: u128 = 2;

    let proof: Array<felt252> = array![
        0xb93b7d65a7e5c7a15def73a61485111e2f630cc8e6683fb98f4d6ca2c7ec96,
        0x91ca2d84afc873630898de633b3041683eea2a5d1d59ae4f3bed3551bb4294,
        0x61c8f928bb4f7b5a3ae252cc4c78f3bdb3442733951a1de12ec01e7a4812a50,
        0x6f0149c2ccc9a95bb64deda90572a912f85139505ea9bcd233f6d16e751af9e,
        0x58e3b614e77af3c7256a74654907b4fe2182daf47da635a81c94629a89595b3
    ];

    return (HASH_ROOT_SET_1, address, amount, timestamp, proof);
}

pub fn get_data_double_claim_bob_alloc() -> (
    felt252, ContractAddress, u128, u128, Array<felt252>, u128, u128, Array<felt252>
) {
    let address: ContractAddress = contract_address_const::<
        0x1234567890abcdef1234567890abcdef12345678
    >();
    let amount1: u128 = 150;
    let amount2: u128 = 150;
    let timestamp1: u128 = 2;
    let timestamp2: u128 = 4;

    let proof1: Array<felt252> = array![
        0xb93b7d65a7e5c7a15def73a61485111e2f630cc8e6683fb98f4d6ca2c7ec96,
        0x91ca2d84afc873630898de633b3041683eea2a5d1d59ae4f3bed3551bb4294,
        0x61c8f928bb4f7b5a3ae252cc4c78f3bdb3442733951a1de12ec01e7a4812a50,
        0x6f0149c2ccc9a95bb64deda90572a912f85139505ea9bcd233f6d16e751af9e,
        0x58e3b614e77af3c7256a74654907b4fe2182daf47da635a81c94629a89595b3
    ];
    let proof2: Array<felt252> = array![
        0xb93b7d65a7e5c7a15def73a61485111e2f630cc8e6683fb98f4d6ca2c7ec96,
        0x91ca2d84afc873630898de633b3041683eea2a5d1d59ae4f3bed3551bb4294,
        0x61c8f928bb4f7b5a3ae252cc4c78f3bdb3442733951a1de12ec01e7a4812a50,
        0x6f0149c2ccc9a95bb64deda90572a912f85139505ea9bcd233f6d16e751af9e,
        0x58e3b614e77af3c7256a74654907b4fe2182daf47da635a81c94629a89595b3
    ];

    return (HASH_ROOT_SET_1, address, amount1, timestamp1, proof1, amount2, timestamp2, proof2);
}

pub fn get_combined_data_bob_alloc() -> (felt252, ContractAddress, u128, u128, Array<felt252>) {
    let address: ContractAddress = contract_address_const::<
        0x1234567890abcdef1234567890abcdef12345678
    >();
    let amount: u128 = 150;
    let timestamp: u128 = 2;

    let proof: Array<felt252> = array![
        0xb93b7d65a7e5c7a15def73a61485111e2f630cc8e6683fb98f4d6ca2c7ec96,
        0x91ca2d84afc873630898de633b3041683eea2a5d1d59ae4f3bed3551bb4294,
        0x61c8f928bb4f7b5a3ae252cc4c78f3bdb3442733951a1de12ec01e7a4812a50,
        0x6f0149c2ccc9a95bb64deda90572a912f85139505ea9bcd233f6d16e751af9e,
        0x58e3b614e77af3c7256a74654907b4fe2182daf47da635a81c94629a89595b3,
        0x57e12be54078fb13aef7df28595941bab33c77cc04b5c74069221be888b182e
    ];

    return (HASH_ROOT_SET_2, address, amount, timestamp, proof);
}

pub fn get_combined_data_alice_alloc() -> (felt252, ContractAddress, u128, u128, Array<felt252>) {
    let address: ContractAddress = contract_address_const::<
        0xabcdefabcdefabcdefabcdefabcdefabcdefabc
    >();
    let amount: u128 = 800;
    let timestamp: u128 = 13;

    let proof: Array<felt252> = array![
        0x67325b9f9f8c14bc7f6e5e61e18beb7e2e413c8045b3e66474b1a9da48675ff,
        0x22cb6a40bcd35b2f143d4fa3556ab6d161bd2d8476fd9113ff12d217a4453d3,
        0x2e355a04aa50c953c458afd1716299e77d8df356f2a3f47a59668a13075f22a,
        0x345b1e11be2309ccf90b7ec5b05b5987a1c4fd9ba232c17918b24144f022666,
        0x427e1a5312507023d5cccfca1919876d1cba3c19278bed6b380722b7c86307c,
        0x3820e57b614f240d6fd07351258ad6f626c5326cf78b8fa6dd54d59e243b0ae
    ];

    return (HASH_ROOT_SET_2, address, amount, timestamp, proof);
}

pub fn get_allocs_first_wave_john() -> (
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
    Array<felt252>,
    Array<felt252>,
    Array<felt252>,
    Array<felt252>
) {
    let address: ContractAddress = contract_address_const::<
        0xabcdefabcdef1234567890abcdef1234567890ab
    >();

    // alloc 1
    let amount1: u128 = 700;
    let timestamp1: u128 = 0x6;
    let proof1: Array<felt252> = array![
        0x46d60ec37dc40ed4b156d95958274dbfb7fce273065825eb65c32d4979eb9c1,
        0x53b3b627301bebe2ba22317925fa58814e752a9c2af25f7508a6c2546afb4af,
        0x5c317f3f86b325fe1aed8103dafccaaafbc0f49ddaeb163b72c574abdcd43ec,
        0x6f0149c2ccc9a95bb64deda90572a912f85139505ea9bcd233f6d16e751af9e,
        0x58e3b614e77af3c7256a74654907b4fe2182daf47da635a81c94629a89595b3
    ];

    // alloc 2
    let amount2: u128 = 900;
    let timestamp2: u128 = 17;
    let proof2: Array<felt252> = array![
        0x1a6e2d03ba7596b143278be0ad0a83e1a2523ee888c75c8b19343509f07e821,
        0x17105708ca2b9efbf1a463b90528c603ed7a23a9493cf65686e36675f05f2f3,
        0x193077a554a313efa6387d3edc11c60a34be8181b69d599e064b26f099b4949,
        0x21ff61c9c9e525d2039631f6fa2e0a34b9de20db17438d049bd369804abb59,
        0x58e3b614e77af3c7256a74654907b4fe2182daf47da635a81c94629a89595b3
    ];

    // alloc 3
    let amount3: u128 = 2500;
    let timestamp3: u128 = 0x18;
    let proof3: Array<felt252> = array![
        0x441810f7802690cd80d39973ba93d97a5326446ebfc01cb6e0eefb4c08e8247,
        0x39f4c300dccca2d76be4ea9951a27045a19c2e647960625755e2b1dd8857658,
        0x59ceaf2eada3198f6fd5ab51d9c374d5a8f0f0d057bba27ca7fb84d319413cb,
        0x130aafa443c502bee0cae358d7d11bef44bf93e8d87df7f3958c11357d989df,
        0x4da2c390813b19356e461046c235956b07a8cd8e914d64811d1c2293a718299
    ];

    // alloc 4 of the second wave
    let amount4: u128 = 287;
    let timestamp4: u128 = 0xE;
    let proof4: Array<felt252> = array![
        0x47ea9d24e7603c67574fc78a104fa2cc5cb7f4511f343b601b640c0ea6ef565,
        0x3b832e149066bee8ec296d905fab1d1200714894a80216e5de233435066f09c,
        0x2e355a04aa50c953c458afd1716299e77d8df356f2a3f47a59668a13075f22a,
        0x345b1e11be2309ccf90b7ec5b05b5987a1c4fd9ba232c17918b24144f022666,
        0x427e1a5312507023d5cccfca1919876d1cba3c19278bed6b380722b7c86307c,
        0x3820e57b614f240d6fd07351258ad6f626c5326cf78b8fa6dd54d59e243b0ae
    ];

    return (
        HASH_ROOT_SET_1,
        HASH_ROOT_SET_2,
        address,
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
        proof3,
        proof4
    );
}
