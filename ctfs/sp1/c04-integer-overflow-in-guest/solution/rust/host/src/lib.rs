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
    let (public_values_hex, proof_hex) = prove_fixture(&client, &pk, &vk, &honest_values);
    let program_vkey = vk.bytes32();

    println!("PROGRAM_VKEY={program_vkey}");
    println!("PUBLIC_VALUES=0x{public_values_hex}");
    println!("PROOF_BYTES=0x{proof_hex}");

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
             bytes internal constant PUBLIC_VALUES = hex\"{public_values_hex}\";\n\
             bytes internal constant PROOF = hex\"{proof_hex}\";\n\
         }}\n",
        honest_recipient = address_literal(HONEST_RECIPIENT),
        honest_price = HONEST_PRICE,
        honest_quantity = HONEST_QUANTITY,
        honest_total = HONEST_TOTAL,
        program_vkey = program_vkey,
        public_values_hex = public_values_hex,
        proof_hex = proof_hex
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

fn prove_fixture(
    client: &CpuProver,
    pk: &SP1ProvingKey,
    vk: &SP1VerifyingKey,
    public_values: &PublicValuesV1,
) -> (String, String) {
    assert!(
        is_guest_accepted_claim(public_values),
        "solution host only proves non-overflowing purchase claims"
    );

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
        challenge_root().join("solution/foundry/test/fixtures/IntegerOverflowFixture.sol");
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

    #[test]
    fn host_policy_accepts_the_honest_fixture() {
        assert!(is_guest_accepted_claim(&honest_public_values()));
    }
}
