use std::fs;
use std::path::PathBuf;

use common::{
    attacker_public_values, honest_public_values, ATTACKER_AMOUNT, ATTACKER_RECIPIENT, HONEST_AMOUNT, HONEST_RECIPIENT,
};
use sp1_sdk::{CpuProver, HashableKey, Prover, ProverClient, SP1Stdin};

const ELF_BASE_PATH: &str = "../target/elf-compilation/riscv32im-succinct-zkvm-elf/release";
const STARTER_FIXTURE_PATH: &str = "../../foundry/test/fixtures/ArbitraryVerificationKeyFixture.sol";
const SOLUTION_FIXTURE_PATH: &str = "../../../solution/foundry/test/fixtures/ArbitraryVerificationKeyFixture.sol";

fn load_guest_elf(program_name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ELF_BASE_PATH).join(program_name);
    fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read guest ELF at {}: {err}\nrun `cargo prove build` in ../ first",
            path.display()
        )
    })
}

fn main() {
    let client = ProverClient::builder().mock().build();

    let honest_elf = load_guest_elf("guest");
    let attacker_elf = load_guest_elf("attacker");

    let honest_values = honest_public_values();
    let (honest_program_vkey, honest_public_values_hex, honest_proof_hex) =
        prove_fixture(&client, &honest_elf, &honest_values);

    let attacker_values = attacker_public_values();
    let (attacker_program_vkey, attacker_public_values_hex, attacker_proof_hex) =
        prove_fixture(&client, &attacker_elf, &attacker_values);

    println!("HONEST_PROGRAM_VKEY={honest_program_vkey}");
    println!("HONEST_PUBLIC_VALUES=0x{honest_public_values_hex}");
    println!("HONEST_PROOF_BYTES=0x{honest_proof_hex}");
    println!("ATTACKER_PROGRAM_VKEY={attacker_program_vkey}");
    println!("ATTACKER_PUBLIC_VALUES=0x{attacker_public_values_hex}");
    println!("ATTACKER_PROOF_BYTES=0x{attacker_proof_hex}");

    let solidity = format!(
        "// SPDX-License-Identifier: MIT\n\
         pragma solidity ^0.8.24;\n\
         \n\
         library ArbitraryVerificationKeyFixture {{\n\
             address internal constant HONEST_RECIPIENT = {honest_recipient};\n\
             uint64 internal constant HONEST_AMOUNT = {honest_amount};\n\
             bytes32 internal constant HONEST_PROGRAM_VKEY = {honest_program_vkey};\n\
             bytes internal constant HONEST_PUBLIC_VALUES = hex\"{honest_public_values_hex}\";\n\
             bytes internal constant HONEST_PROOF = hex\"{honest_proof_hex}\";\n\
         \n\
             address internal constant ATTACKER_RECIPIENT = {attacker_recipient};\n\
             uint64 internal constant ATTACKER_AMOUNT = {attacker_amount};\n\
             bytes32 internal constant ATTACKER_PROGRAM_VKEY = {attacker_program_vkey};\n\
             bytes internal constant ATTACKER_PUBLIC_VALUES = hex\"{attacker_public_values_hex}\";\n\
             bytes internal constant ATTACKER_PROOF = hex\"{attacker_proof_hex}\";\n\
         }}\n",
        honest_recipient = address_literal(HONEST_RECIPIENT),
        honest_amount = HONEST_AMOUNT,
        honest_program_vkey = honest_program_vkey,
        honest_public_values_hex = honest_public_values_hex,
        honest_proof_hex = honest_proof_hex,
        attacker_recipient = address_literal(ATTACKER_RECIPIENT),
        attacker_amount = ATTACKER_AMOUNT,
        attacker_program_vkey = attacker_program_vkey,
        attacker_public_values_hex = attacker_public_values_hex,
        attacker_proof_hex = attacker_proof_hex
    );

    write_fixture(STARTER_FIXTURE_PATH, &solidity);
    write_fixture(SOLUTION_FIXTURE_PATH, &solidity);
}

fn prove_fixture(client: &CpuProver, elf: &[u8], public_values: &common::PublicValuesV1) -> (String, String, String) {
    let (pk, vk) = client.setup(elf);

    let mut stdin = SP1Stdin::new();
    stdin.write(public_values);

    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("proof generation should succeed");

    client
        .verify(&proof, &vk)
        .expect("proof verification should succeed");

    (
        vk.bytes32().to_string(),
        hex_encode(&proof.public_values.to_vec()),
        hex_encode(&proof.bytes())
    )
}

fn write_fixture(relative_path: &str, solidity: &str) {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    if let Some(parent) = fixture_path.parent() {
        fs::create_dir_all(parent).expect("failed to create fixture directory");
    }
    fs::write(&fixture_path, solidity).expect("failed to write Solidity fixture");
    println!("WROTE_FIXTURE={}", fixture_path.display());
}

fn address_literal(bytes: [u8; 20]) -> String {
    format!("0x{}", hex_encode(&bytes))
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
