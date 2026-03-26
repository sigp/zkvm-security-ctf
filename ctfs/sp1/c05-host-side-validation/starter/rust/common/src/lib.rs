// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Fields that are validated by the guest program.
pub const EXPECTED_CHAIN_ID: u64 = 1;
pub const EXPECTED_VERSION: u32 = 1;

/// Canonical claim values used by the honest host.
pub const HONEST_CONTEXT_HASH: [u8; 32] = [0x22; 32];
pub const HONEST_RECIPIENT: [u8; 20] = [0x33; 20];
pub const HONEST_AMOUNT: u64 = 100;
pub const HONEST_NONCE: u64 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicValuesV1 {
    pub chain_id: u64,
    pub context_hash: [u8; 32],
    pub recipient: [u8; 20],
    pub amount: u64,
    pub nonce: u64,
    pub version: u32,
}

pub fn honest_public_values() -> PublicValuesV1 {
    PublicValuesV1 {
        chain_id: EXPECTED_CHAIN_ID,
        context_hash: HONEST_CONTEXT_HASH,
        recipient: HONEST_RECIPIENT,
        amount: HONEST_AMOUNT,
        nonce: HONEST_NONCE,
        version: EXPECTED_VERSION,
    }
}

/// Validates the fields that the guest program checks.
/// Returns true if chain_id and version match expected values.
pub fn is_valid_claim(public_values: &PublicValuesV1) -> bool {
    public_values.chain_id == EXPECTED_CHAIN_ID && public_values.version == EXPECTED_VERSION
}

/// Returns true only if all fields match the canonical honest claim.
pub fn is_honest_claim(public_values: &PublicValuesV1) -> bool {
    public_values == &honest_public_values()
}

pub fn serialize_public_values(public_values: &PublicValuesV1) -> Vec<u8> {
    let mut out = Vec::with_capacity(80);
    out.extend_from_slice(&public_values.chain_id.to_le_bytes());
    out.extend_from_slice(&public_values.context_hash);
    out.extend_from_slice(&public_values.recipient);
    out.extend_from_slice(&public_values.amount.to_le_bytes());
    out.extend_from_slice(&public_values.nonce.to_le_bytes());
    out.extend_from_slice(&public_values.version.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn honest_claim_passes_guest_validation() {
        assert!(is_valid_claim(&honest_public_values()));
    }

    #[test]
    fn honest_claim_matches_full_policy() {
        assert!(is_honest_claim(&honest_public_values()));
    }

    #[test]
    fn modified_claim_with_valid_chain_and_version_passes_guest_validation() {
        let modified = PublicValuesV1 {
            chain_id: EXPECTED_CHAIN_ID,
            context_hash: [0xaa; 32],
            recipient: [0x99; 20],
            amount: 999_999,
            nonce: 42,
            version: EXPECTED_VERSION,
        };
        // Guest validation passes...
        assert!(is_valid_claim(&modified));
        // ...but it's not the honest claim
        assert!(!is_honest_claim(&modified));
    }

    #[test]
    fn wrong_chain_id_fails_guest_validation() {
        let wrong_chain = PublicValuesV1 {
            chain_id: 999,
            ..honest_public_values()
        };
        assert!(!is_valid_claim(&wrong_chain));
    }

    #[test]
    fn wrong_version_fails_guest_validation() {
        let wrong_version = PublicValuesV1 {
            version: 99,
            ..honest_public_values()
        };
        assert!(!is_valid_claim(&wrong_version));
    }

    #[test]
    fn public_values_layout_matches_the_solidity_decoder() {
        assert_eq!(serialize_public_values(&honest_public_values()).len(), 80);
    }
}
