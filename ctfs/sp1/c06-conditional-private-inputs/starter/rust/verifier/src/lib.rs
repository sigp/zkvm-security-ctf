// SPDX-License-Identifier: MIT

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
    use common::{
        honest_private_inputs, ATTACKER_AMOUNT, ATTACKER_RECIPIENT, EXPECTED_AMOUNT,
        EXPECTED_RECIPIENT,
    };

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
    #[ignore = "exercise scaffold: remove ignore after filling in the TODOs"]
    fn exercise_test_mode_skips_validation() {
        // TODO: Replace the honest input with a malicious private input that sets `is_test = true`
        // and commits attacker-controlled claim fields.
        let fixture = host::prove_custom(&honest_private_inputs());
        let mut verifier = BalanceVerifier::new(fixture.vk.clone());

        let claim = verifier
            .submit(&fixture.proof)
            .expect("proof should verify");

        // TODO: Replace these honest-claim assertions with attacker-controlled values after you
        // generate the exploit proof.
        assert_eq!(claim.recipient, EXPECTED_RECIPIENT);
        assert_eq!(claim.amount, EXPECTED_AMOUNT);
        assert_eq!(verifier.balance_of(ATTACKER_RECIPIENT), ATTACKER_AMOUNT);
    }
}
