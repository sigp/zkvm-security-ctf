// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

use common::{
    honest_public_values, is_guest_accepted_claim, PublicValuesV1, HONEST_PRICE, HONEST_QUANTITY,
    HONEST_RECIPIENT, HONEST_TOTAL,
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

    let program_vkey = vk.bytes32();

    println!("PROGRAM_VKEY={program_vkey}");
    println!("HONEST_PUBLIC_VALUES=0x{honest_public_values_hex}");
    println!("HONEST_PROOF_BYTES=0x{honest_proof_hex}");

    let solidity = format!(
        "// SPDX-License-Identifier: MIT\n\
         pragma solidity ^0.8.24;\n\
         \n\
         library IntegerOverflowFixture {{\n\
             address internal constant HONEST_RECIPIENT = {honest_recipient};\n\
             uint64 internal constant HONEST_PRICE = {honest_price};\n\
             uint64 internal constant HONEST_QUANTITY = {honest_quantity};\n\
             uint64 internal constant HONEST_TOTAL = {honest_total};\n\
             bytes32 internal constant PROGRAM_VKEY = {program_vkey};\n\
             bytes internal constant HONEST_PUBLIC_VALUES = hex\"{honest_public_values_hex}\";\n\
             bytes internal constant HONEST_PROOF = hex\"{honest_proof_hex}\";\n\
         }}\n",
        honest_recipient = address_literal(HONEST_RECIPIENT),
        honest_price = HONEST_PRICE,
        honest_quantity = HONEST_QUANTITY,
        honest_total = HONEST_TOTAL,
        program_vkey = program_vkey,
        honest_public_values_hex = honest_public_values_hex,
        honest_proof_hex = honest_proof_hex,
    );

    write_fixture(&solidity);
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

/// Generates a proof for arbitrary public values that satisfy the vulnerable guest checks.
pub fn prove_custom(public_values: &PublicValuesV1) -> (String, String, String) {
    assert!(
        is_guest_accepted_claim(public_values),
        "guest will reject claims whose total does not match wrapping arithmetic"
    );

    let client = ProverClient::builder().cpu().build();
    let elf = load_guest_elf();
    let (pk, vk) = client.setup(&elf);

    let (public_values_hex, proof_hex) = prove_fixture(&client, &pk, &vk, public_values);
    let program_vkey = vk.bytes32();

    (program_vkey, public_values_hex, proof_hex)
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

fn write_fixture(solidity: &str) {
    let fixture_path =
        challenge_root().join("starter/foundry/test/fixtures/IntegerOverflowFixture.sol");
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
        .expect("failed to resolve c04 challenge root")
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

#[cfg(test)]
mod tests {
    use super::*;
    use common::{checked_total, wrapping_total, EXPECTED_CHAIN_ID, EXPECTED_VERSION, FIXED_CONTEXT_HASH};

    #[test]
    fn honest_claim_passes_validation() {
        assert!(is_guest_accepted_claim(&honest_public_values()));
    }

    #[test]
    fn overflow_claim_with_wrapped_total_passes() {
        let custom = PublicValuesV1 {
            chain_id: EXPECTED_CHAIN_ID,
            context_hash: FIXED_CONTEXT_HASH,
            recipient: [0xcc; 20],
            price: u64::MAX,
            quantity: 2,
            total: wrapping_total(u64::MAX, 2),
            version: EXPECTED_VERSION,
        };
        assert!(checked_total(custom.price, custom.quantity).is_none());
        assert!(is_guest_accepted_claim(&custom));
    }
}
