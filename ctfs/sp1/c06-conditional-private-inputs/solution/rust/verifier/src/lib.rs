use std::collections::HashMap;

use anyhow::{anyhow, Result};
use common::ClaimPublicValues;
use sp1_sdk::{Prover, ProverClient, SP1ProofWithPublicValues, SP1VerifyingKey};

pub struct BalanceVerifier {
    trusted_vk: SP1VerifyingKey,
    balances: HashMap<[u8; 20], u64>,
}

impl BalanceVerifier {
    pub fn new(trusted_vk: SP1VerifyingKey) -> Self {
        Self {
            trusted_vk,
            balances: HashMap::new(),
        }
    }

    pub fn submit(&mut self, proof: &SP1ProofWithPublicValues) -> Result<ClaimPublicValues> {
        let client = ProverClient::builder().mock().build();
        client
            .verify(proof, &self.trusted_vk)
            .map_err(|err| anyhow!("proof verification failed: {err}"))?;

        let claim = host::decode_claim(proof);
        let balance = self.balances.entry(claim.recipient).or_default();
        *balance = balance
            .checked_add(claim.amount)
            .ok_or_else(|| anyhow!("balance overflow while crediting recipient"))?;

        Ok(claim)
    }

    pub fn balance_of(&self, recipient: [u8; 20]) -> u64 {
        self.balances.get(&recipient).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{exploit_private_inputs, EXPECTED_AMOUNT, EXPECTED_RECIPIENT};

    #[test]
    fn honest_proof_credits_the_canonical_recipient() {
        let fixture = host::prove_honest();
        let mut verifier = BalanceVerifier::new(fixture.vk.clone());

        let claim = verifier
            .submit(&fixture.proof)
            .expect("honest proof should verify");

        assert_eq!(claim, fixture.claim);
        assert_eq!(verifier.balance_of(EXPECTED_RECIPIENT), EXPECTED_AMOUNT);
    }

    #[test]
    fn exploit_private_inputs_are_rejected_before_the_verifier_sees_a_proof() {
        assert!(host::try_prove(&exploit_private_inputs()).is_err());
    }
}
