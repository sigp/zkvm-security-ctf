// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

use common::{
    foreign_public_values, honest_public_values, PublicValuesV1, FIXED_AMOUNT, FIXED_RECIPIENT,
    FOREIGN_CHAIN_ID, HONEST_CHAIN_ID,
};
use sp1_sdk::{
    CpuProver, HashableKey, Prover, ProverClient, SP1ProvingKey, SP1Stdin, SP1VerifyingKey,
};

const ELF_RELATIVE_PATH: &str =
    "../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/guest";

pub fn run() {
    let client = ProverClient::builder().cpu().build();
    let elf = load_guest_elf();
    let (pk, vk) = client.setup(&elf);

    let honest_values = honest_public_values();
    let (honest_public_values_hex, honest_proof_hex) =
        prove_fixture(&client, &pk, &vk, &honest_values);

    let foreign_values = foreign_public_values();
    let (foreign_public_values_hex, foreign_proof_hex) =
        prove_fixture(&client, &pk, &vk, &foreign_values);

    let program_vkey = vk.bytes32();

    println!("PROGRAM_VKEY={program_vkey}");
    println!("HONEST_PUBLIC_VALUES=0x{honest_public_values_hex}");
    println!("HONEST_PROOF_BYTES=0x{honest_proof_hex}");
    println!("FOREIGN_PUBLIC_VALUES=0x{foreign_public_values_hex}");
    println!("FOREIGN_PROOF_BYTES=0x{foreign_proof_hex}");

    let solidity = format!(
        "// SPDX-License-Identifier: MIT\n\
         pragma solidity ^0.8.24;\n\
         \n\
         library UnconstrainedVerifierFixture {{\n\
             address internal constant FIXED_RECIPIENT = {fixed_recipient};\n\
             uint64 internal constant FIXED_AMOUNT = {fixed_amount};\n\
             uint64 internal constant HONEST_CHAIN_ID = {honest_chain_id};\n\
             uint64 internal constant FOREIGN_CHAIN_ID = {foreign_chain_id};\n\
             bytes32 internal constant PROGRAM_VKEY = {program_vkey};\n\
             bytes internal constant HONEST_PUBLIC_VALUES = hex\"{honest_public_values_hex}\";\n\
             bytes internal constant HONEST_PROOF = hex\"{honest_proof_hex}\";\n\
             bytes internal constant FOREIGN_PUBLIC_VALUES = hex\"{foreign_public_values_hex}\";\n\
             bytes internal constant FOREIGN_PROOF = hex\"{foreign_proof_hex}\";\n\
         }}\n",
        fixed_recipient = address_literal(FIXED_RECIPIENT),
        fixed_amount = FIXED_AMOUNT,
        honest_chain_id = HONEST_CHAIN_ID,
        foreign_chain_id = FOREIGN_CHAIN_ID,
        program_vkey = program_vkey,
        honest_public_values_hex = honest_public_values_hex,
        honest_proof_hex = honest_proof_hex,
        foreign_public_values_hex = foreign_public_values_hex,
        foreign_proof_hex = foreign_proof_hex
    );

    write_fixture("starter", &solidity);
    write_fixture("solution", &solidity);
}

fn load_guest_elf() -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ELF_RELATIVE_PATH);
    fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read guest ELF at {}: {err}\nrun `cargo prove build` in ../ first",
            path.display()
        )
    })
}

fn prove_fixture(
    client: &CpuProver,
    pk: &SP1ProvingKey,
    vk: &SP1VerifyingKey,
    public_values: &PublicValuesV1,
) -> (String, String) {
    let mut stdin = SP1Stdin::new();
    stdin.write(public_values);

    let proof = client
        .prove(pk, &stdin)
        .groth16()
        .run()
        .expect("proof generation should succeed");

    client
        .verify(&proof, vk)
        .expect("proof verification should succeed");

    (
        hex_encode(&proof.public_values.to_vec()),
        hex_encode(&proof.bytes()),
    )
}

fn write_fixture(flavor: &str, solidity: &str) {
    let fixture_path = challenge_root()
        .join(flavor)
        .join("foundry/test/fixtures/UnconstrainedVerifierFixture.sol");
    if let Some(parent) = fixture_path.parent() {
        fs::create_dir_all(parent).expect("failed to create fixture directory");
    }
    fs::write(&fixture_path, solidity).expect("failed to write Solidity fixture");
    println!("WROTE_FIXTURE={}", fixture_path.display());
}

fn challenge_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("failed to resolve c03 challenge root")
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
