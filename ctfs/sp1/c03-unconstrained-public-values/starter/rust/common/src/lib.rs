// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

pub const HONEST_CHAIN_ID: u64 = 1;
pub const FOREIGN_CHAIN_ID: u64 = 10;
pub const FIXED_CONTEXT_HASH: [u8; 32] = [0x22; 32];
pub const FIXED_RECIPIENT: [u8; 20] = [0x33; 20];
pub const FIXED_AMOUNT: u64 = 100;
pub const FIXED_NONCE: u64 = 1;
pub const FIXED_VERSION: u32 = 1;

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
    public_values_for_chain(HONEST_CHAIN_ID)
}

pub fn foreign_public_values() -> PublicValuesV1 {
    public_values_for_chain(FOREIGN_CHAIN_ID)
}

pub fn public_values_for_chain(chain_id: u64) -> PublicValuesV1 {
    PublicValuesV1 {
        chain_id,
        context_hash: FIXED_CONTEXT_HASH,
        recipient: FIXED_RECIPIENT,
        amount: FIXED_AMOUNT,
        nonce: FIXED_NONCE,
        version: FIXED_VERSION,
    }
}

pub fn is_canonical_payload_except_chain_id(public_values: &PublicValuesV1) -> bool {
    public_values.context_hash == FIXED_CONTEXT_HASH
        && public_values.recipient == FIXED_RECIPIENT
        && public_values.amount == FIXED_AMOUNT
        && public_values.nonce == FIXED_NONCE
        && public_values.version == FIXED_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_payload_allows_different_chain_ids() {
        assert!(is_canonical_payload_except_chain_id(&honest_public_values()));
        assert!(is_canonical_payload_except_chain_id(
            &foreign_public_values()
        ));
    }

    #[test]
    fn tampering_non_chain_fields_breaks_the_canonical_payload() {
        let mut public_values = honest_public_values();
        public_values.amount += 1;

        assert!(!is_canonical_payload_except_chain_id(&public_values));
    }
}
