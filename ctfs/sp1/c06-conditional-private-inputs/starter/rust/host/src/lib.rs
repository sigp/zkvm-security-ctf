use anyhow::{anyhow, Result};
use common::{honest_private_inputs, ClaimPublicValues, PrivateInputs};
use sp1_sdk::{
    include_elf, HashableKey, Prover, ProverClient, SP1ProofWithPublicValues, SP1Stdin,
    SP1VerifyingKey,
};

const GUEST_ELF: &[u8] = include_elf!("guest");

pub struct ProofFixture {
    pub program_vkey: String,
    pub vk: SP1VerifyingKey,
    pub proof: SP1ProofWithPublicValues,
    pub claim: ClaimPublicValues,
}

pub fn run() {
    let fixture = prove_honest();
    println!("PROGRAM_VKEY={}", fixture.program_vkey);
    println!("PUBLIC_VALUES={:?}", fixture.claim);
    println!("PROOF_MODE=mock");
}

pub fn prove_honest() -> ProofFixture {
    prove_custom(&honest_private_inputs())
}

pub fn prove_custom(inputs: &PrivateInputs) -> ProofFixture {
    try_prove(inputs).expect("proof generation should succeed")
}

pub fn try_prove(inputs: &PrivateInputs) -> Result<ProofFixture> {
    let client = ProverClient::builder().mock().build();
    let (pk, vk) = client.setup(GUEST_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(inputs);

    let proof = client
        .prove(&pk, &stdin)
        .compressed()
        .run()
        .map_err(|err| anyhow!("proof generation failed: {err}"))?;

    client
        .verify(&proof, &vk)
        .map_err(|err| anyhow!("proof verification failed: {err}"))?;

    let claim = decode_claim(&proof);

    Ok(ProofFixture {
        program_vkey: vk.bytes32(),
        vk,
        proof,
        claim,
    })
}

pub fn decode_claim(proof: &SP1ProofWithPublicValues) -> ClaimPublicValues {
    let mut public_values = proof.public_values.clone();
    public_values.read::<ClaimPublicValues>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{canonical_public_values, exploit_private_inputs};

    #[test]
    fn honest_proof_commits_the_canonical_claim() {
        let fixture = prove_honest();
        assert_eq!(fixture.claim, canonical_public_values());
    }

    #[test]
    fn exploit_inputs_generate_a_valid_proof_in_the_starter() {
        let fixture = prove_custom(&exploit_private_inputs());
        assert_eq!(fixture.claim, exploit_private_inputs().claim);
    }
}
