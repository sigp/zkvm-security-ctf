// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

use common::fixed_public_values;
use sp1_sdk::{HashableKey, Prover, ProverClient, SP1Stdin};

const ELF_RELATIVE_PATH: &str = "../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/guest";

fn load_guest_elf() -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ELF_RELATIVE_PATH);
    fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read guest ELF at {}: {err}\nrun `cargo prove build` in ../guest first",
            path.display()
        )
    })
}

fn main() {
    // setup
    let client = ProverClient::builder().cpu().build();
    let elf = load_guest_elf();
    let (pk, vk) = client.setup(&elf);

    let public_values = fixed_public_values();
    let mut stdin = SP1Stdin::new();
    stdin.write(&public_values);

    // create proof
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("proof generation should succeed");

    // verify proof
    client
        .verify(&proof, &vk)
        .expect("proof verification should succeed");

    let program_vkey = vk.bytes32();
    let public_values_hex = hex_encode(&proof.public_values.to_vec());
    let proof_bytes_hex = hex_encode(&proof.bytes());

    println!("PROGRAM_VKEY={program_vkey}");
    println!("PUBLIC_VALUES=0x{public_values_hex}");
    println!("PROOF_BYTES=0x{proof_bytes_hex}");

    // Emit a Solidity fixture file so Foundry tests can import real values directly.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../foundry/test/fixtures/ReplayableProofStarterFixture.sol");
    if let Some(parent) = fixture_path.parent() {
        fs::create_dir_all(parent).expect("failed to create fixture directory");
    }
    let solidity = format!(
        "// SPDX-License-Identifier: MIT\n\
         pragma solidity ^0.8.24;\n\
         \n\
         library ReplayableProofStarterFixture {{\n\
             bytes32 internal constant PROGRAM_VKEY = {program_vkey};\n\
             bytes internal constant PUBLIC_VALUES = hex\"{public_values_hex}\";\n\
             bytes internal constant PROOF = hex\"{proof_bytes_hex}\";\n\
         }}\n"
    );
    fs::write(&fixture_path, solidity).expect("failed to write Solidity fixture");
    println!("WROTE_FIXTURE={}", fixture_path.display());
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}
