import sys
import os

running_path = os.getcwd()
arguments = sys.argv
relative_output_text_path = f"{running_path}/{arguments[1]}"
relative_cairo_test_path = f"{running_path}/{arguments[2]}"

first_merkle_data = f"{relative_output_text_path}/first_merkle_tree_data.txt"
second_merkle_data = f"{relative_output_text_path}/second_merkle_tree_data.txt"
constants_cairo = f"{relative_cairo_test_path}/constants.cairo"

try:
    with open(first_merkle_data, 'r') as my_file:
        data_1 = my_file.read()
    with open(second_merkle_data) as my_file:
        data_2 = my_file.read()
except Exception as e:
    print(e)
    raise SystemExit

def get_all_details(input_string):
    input_string = input_string + 'P'
    root_hash = input_string[:input_string.index('\n')].strip()
    count = 0
    num = 0
    start = 0
    stop = 0
    details = ""
    details_list = []

    for char in input_string:
        if char == ':':
            start = count + 2
        count += 1
        if char == 'P' and start != 0:
            stop = count - 2
            for k in input_string[start:stop]:
                if k == '\n':
                    if num <4:
                        details += '.'
                        num +=1
                        continue
                    else:
                        details += ',\n\t\t'
                        continue
                details+=k
    
            details_list.append(details[:-4].split('.'))
            details = ""
            num = 0
    return [root_hash,details_list]

def to_decimal(hex_value):
    return str(int(hex_value, 16))
        
root_hash_1,all_details_1 = get_all_details(data_1[22:])
root_hash2,all_details_2 = get_all_details(data_2[22:])

test_code =f"""use starknet::{{ContractAddress, contract_address_const}};

pub const MERKLE_ROOT_FIRST_WAVE: felt252 =
    {root_hash_1};

pub const MERKLE_ROOT_SECOND_WAVE: felt252 =
    {root_hash2};

pub fn get_bob_first_wave_allocation() -> (
    felt252, ContractAddress, u128, u128, u128, Array<felt252>
) {{
    let address: ContractAddress = contract_address_const::<
        {all_details_1[0][0]}
    >();
    let amount: u128 = {to_decimal(all_details_1[0][1])};
    let timestamp: u128 = {to_decimal(all_details_1[0][2])};
    let id: u128 = {to_decimal(all_details_1[0][3])};

    let proof: Array<felt252> = array![
        {all_details_1[0][4]}
    ];

    (MERKLE_ROOT_FIRST_WAVE, address, amount, timestamp, id, proof)
}}

pub fn get_bob_second_wave_allocation() -> (
    felt252, ContractAddress, u128, u128, u128, Array<felt252>
) {{
    let address: ContractAddress = contract_address_const::<
        {all_details_2[0][0]}
    >();
    let amount: u128 = {to_decimal(all_details_2[0][1])};
    let timestamp: u128 = {to_decimal(all_details_2[0][2])};
    let id: u128 = {to_decimal(all_details_2[0][3])};

    let proof: Array<felt252> = array![
        {all_details_2[0][4]}
    ];

    (MERKLE_ROOT_SECOND_WAVE, address, amount, timestamp, id, proof)
}}

pub fn get_alice_second_wave_allocation() -> (
    felt252, ContractAddress, u128, u128, u128, Array<felt252>
) {{
    let address: ContractAddress = contract_address_const::<
        {all_details_2[41][0]}
    >();
    let amount: u128 = {to_decimal(all_details_2[41][1])};
    let timestamp: u128 = {to_decimal(all_details_2[41][2])};
    let id: u128 = {to_decimal(all_details_2[41][3])};

    let proof: Array<felt252> = array![
        {all_details_2[41][4]}
    ];

    (MERKLE_ROOT_SECOND_WAVE, address, amount, timestamp, id, proof)
}}

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
) {{
    let address: ContractAddress = contract_address_const::<
        {all_details_1[6][0]}
    >();

    // Allocation 1
    let amount1: u128 = {to_decimal(all_details_1[6][1])};
    let timestamp1: u128 = {all_details_1[6][2]};
    let id_1: u128 = {to_decimal(all_details_1[6][3])};
    let proof1: Array<felt252> = array![
        {all_details_1[6][4]}
    ];

    // Allocation 2
    let amount2: u128 = {to_decimal(all_details_1[15][1])};
    let timestamp2: u128 = {to_decimal(all_details_1[15][2])};
    let id_2: u128 = {to_decimal(all_details_1[15][3])};
    let proof2: Array<felt252> = array![
        {all_details_1[15][4]}
    ];

    // Allocation 3
    let amount3: u128 = {to_decimal(all_details_1[24][1])};
    let timestamp3: u128 = {all_details_1[24][2]};
    let id_3: u128 = {to_decimal(all_details_1[24][3])};
    let proof3: Array<felt252> = array![
        {all_details_1[24][4]}
    ];

    // Allocation 4 of the second wave
    let amount4: u128 = {to_decimal(all_details_2[42][1])};
    let timestamp4: u128 = {all_details_2[42][2]};
    let id_4: u128 = {to_decimal(all_details_2[42][3])};
    let proof4: Array<felt252> = array![
        {all_details_2[42][4]}
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
}}
"""
try:
    with open(constants_cairo, "w") as my_file:
        my_file.write(test_code)
        print("test file updated!")
except Exception as e:
    print(e)
