use merkle_tree_rust::{Allocation, MerkleTree};
use serde_json::from_reader;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

fn main() {
    // Step 1: Process the first wave and create the first Merkle tree
    let file = File::open("tests/mock_allocations_first_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let allocations_first_wave: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    let tree_first_wave = MerkleTree::new(allocations_first_wave.clone());
    let root_hash_first_wave = tree_first_wave.root.value.to_string();

    // Step 2: Write the first Merkle tree data to a text file
    let mut output_file_first = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tests/first_merkle_tree_data.txt")
        .expect("Unable to open or create the file");

    writeln!(
        output_file_first,
        "Root Hash First Wave: {}\n",
        root_hash_first_wave
    )
    .expect("Failed to write root hash");

    for allocation in allocations_first_wave.iter() {
        let proof = tree_first_wave
            .build_address_calldata(
                &allocation.address,
                allocation.amount,
                &allocation.timestamp,
            )
            .expect("Proof generation failed");

        writeln!(
            output_file_first,
            "Proof for address {}:",
            allocation.address
        )
        .expect("Failed to write proof header");

        for p in proof {
            if let Ok(decimal_value) = u64::from_str_radix(p.trim_start_matches("0x"), 16) {
                writeln!(output_file_first, "{}", decimal_value).expect("Failed to write proof");
            } else {
                writeln!(output_file_first, "{}", p).expect("Failed to write proof");
            }
        }
        writeln!(output_file_first).expect("Failed to write newline");
    }

    println!("First Merkle tree data has been stored in tests/first_merkle_tree_data.txt");

    // Step 3: Process the second wave, merge it with the first wave, and create the second Merkle tree
    let file = File::open("tests/mock_allocations_second_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let allocations_second_wave: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    // Merge the first wave with the second wave allocations
    let mut combined_allocations = allocations_first_wave.clone();
    combined_allocations.extend(allocations_second_wave.clone());

    let tree_second_wave = MerkleTree::new(combined_allocations.clone());
    let root_hash_second_wave = tree_second_wave.root.value.to_string();

    // Step 4: Write the second Merkle tree data to a separate text file
    let mut output_file_second = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tests/second_merkle_tree_data.txt")
        .expect("Unable to open or create the file");

    writeln!(
        output_file_second,
        "Root Hash Second Wave: {}\n",
        root_hash_second_wave
    )
    .expect("Failed to write root hash");

    for allocation in combined_allocations.iter() {
        let proof = tree_second_wave
            .build_address_calldata(
                &allocation.address,
                allocation.amount,
                &allocation.timestamp,
            )
            .expect("Proof generation failed");

        writeln!(
            output_file_second,
            "Proof for address {}:",
            allocation.address
        )
        .expect("Failed to write proof header");

        for p in proof {
            if let Ok(decimal_value) = u64::from_str_radix(p.trim_start_matches("0x"), 16) {
                writeln!(output_file_second, "{}", decimal_value).expect("Failed to write proof");
            } else {
                writeln!(output_file_second, "{}", p).expect("Failed to write proof");
            }
        }
        writeln!(output_file_second).expect("Failed to write newline");
    }

    println!("Second Merkle tree data has been stored in tests/second_merkle_tree_data.txt");
}
