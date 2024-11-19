use merkle_tree::core::allocation::Allocation;
use merkle_tree::core::merkle_tree::MerkleTree;
use serde_json::from_reader;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

fn main() {
    // Step 1: Load first wave of allocations
    let file = File::open("data/mock_allocations_first_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let allocations_first_wave: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    // Step 2: Create first Merkle tree
    let tree_first_wave = MerkleTree::new(allocations_first_wave.clone());
    let root_hash_first_wave = tree_first_wave.root.value.to_string();

    // Step 3: Write first wave Merkle tree data
    let mut output_file_first = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output/first_merkle_tree_data.txt")
        .expect("Unable to open or create file");

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
                allocation.id,
            )
            .expect("Proof generation failed");

        writeln!(
            output_file_first,
            "Proof for address {}:",
            allocation.address
        )
        .expect("Failed to write proof header");

        for p in proof {
            writeln!(output_file_first, "{}", p).expect("Failed to write proof");
        }
        writeln!(output_file_first).expect("Failed to write newline");
    }

    println!("First Merkle tree data saved in tests/first_merkle_tree_data.txt");

    // Step 4: Load second wave of allocations
    let file = File::open("data/mock_allocations_second_wave.json").expect("File not found");
    let reader = BufReader::new(file);
    let allocations_second_wave: Vec<Allocation> = from_reader(reader).expect("Error reading JSON");

    // Step 5: Merge and create second Merkle tree
    let mut combined_allocations = allocations_first_wave.clone();
    combined_allocations.extend(allocations_second_wave.clone());

    let tree_second_wave = MerkleTree::new(combined_allocations.clone());
    let root_hash_second_wave = tree_second_wave.root.value.to_string();

    // Step 6: Write second wave Merkle tree data
    let mut output_file_second = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output/second_merkle_tree_data.txt")
        .expect("Unable to open or create file");

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
                allocation.id,
            )
            .expect("Proof generation failed");

        writeln!(
            output_file_second,
            "Proof for address {}:",
            allocation.address
        )
        .expect("Failed to write proof header");

        for p in proof {
            writeln!(output_file_second, "{}", p).expect("Failed to write proof");
        }
        writeln!(output_file_second).expect("Failed to write newline");
    }

    println!("Second Merkle tree data saved in tests/second_merkle_tree_data.txt");
}
