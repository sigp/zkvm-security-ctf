use serde::{Deserialize, Serialize};

pub const EXPECTED_CHAIN_ID: u64 = 1;
pub const EXPECTED_CONTEXT_HASH: [u8; 32] = [0x22; 32];
pub const EXPECTED_RECIPIENT: [u8; 20] = [0x33; 20];
pub const EXPECTED_AMOUNT: u64 = 100;
pub const EXPECTED_NONCE: u64 = 1;
pub const EXPECTED_VERSION: u32 = 1;

pub const ATTACKER_CONTEXT_HASH: [u8; 32] = [0xaa; 32];
pub const ATTACKER_RECIPIENT: [u8; 20] = [0x99; 20];
pub const ATTACKER_AMOUNT: u64 = 1_000_000;
pub const ATTACKER_NONCE: u64 = 99;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimPublicValues {
    pub chain_id: u64,
    pub context_hash: [u8; 32],
    pub recipient: [u8; 20],
    pub amount: u64,
    pub nonce: u64,
    pub version: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateInputs {
    pub is_test: bool,
    pub claim: ClaimPublicValues,
}

pub fn canonical_public_values() -> ClaimPublicValues {
    ClaimPublicValues {
        chain_id: EXPECTED_CHAIN_ID,
        context_hash: EXPECTED_CONTEXT_HASH,
        recipient: EXPECTED_RECIPIENT,
        amount: EXPECTED_AMOUNT,
        nonce: EXPECTED_NONCE,
        version: EXPECTED_VERSION,
    }
}

pub fn honest_private_inputs() -> PrivateInputs {
    PrivateInputs {
        is_test: false,
        claim: canonical_public_values(),
    }
}

pub fn exploit_public_values() -> ClaimPublicValues {
    ClaimPublicValues {
        chain_id: EXPECTED_CHAIN_ID,
        context_hash: ATTACKER_CONTEXT_HASH,
        recipient: ATTACKER_RECIPIENT,
        amount: ATTACKER_AMOUNT,
        nonce: ATTACKER_NONCE,
        version: EXPECTED_VERSION,
    }
}

pub fn exploit_private_inputs() -> PrivateInputs {
    PrivateInputs {
        is_test: true,
        claim: exploit_public_values(),
    }
}

pub fn is_canonical_claim(claim: &ClaimPublicValues) -> bool {
    claim == &canonical_public_values()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn honest_inputs_match_the_canonical_claim() {
        assert_eq!(honest_private_inputs().claim, canonical_public_values());
    }

    #[test]
    fn exploit_inputs_are_not_canonical() {
        assert!(!is_canonical_claim(&exploit_public_values()));
    }

    #[test]
    fn exploit_inputs_only_change_private_control_flow_and_claim_contents() {
        let exploit = exploit_private_inputs();
        assert!(exploit.is_test);
        assert_eq!(exploit.claim.chain_id, EXPECTED_CHAIN_ID);
        assert_eq!(exploit.claim.version, EXPECTED_VERSION);
    }
}
