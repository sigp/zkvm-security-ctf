// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

pub const EXPECTED_CHAIN_ID: u64 = 1;
pub const EXPECTED_VERSION: u32 = 1;
pub const FIXED_CONTEXT_HASH: [u8; 32] = [0x22; 32];
pub const HONEST_RECIPIENT: [u8; 20] = [0x33; 20];
pub const HONEST_PRICE: u64 = 10;
pub const HONEST_QUANTITY: u64 = 3;
pub const HONEST_TOTAL: u64 = 30;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicValuesV1 {
    pub chain_id: u64,
    pub context_hash: [u8; 32],
    pub recipient: [u8; 20],
    pub price: u64,
    pub quantity: u64,
    pub total: u64,
    pub version: u32,
}

pub fn honest_public_values() -> PublicValuesV1 {
    PublicValuesV1 {
        chain_id: EXPECTED_CHAIN_ID,
        context_hash: FIXED_CONTEXT_HASH,
        recipient: HONEST_RECIPIENT,
        price: HONEST_PRICE,
        quantity: HONEST_QUANTITY,
        total: HONEST_TOTAL,
        version: EXPECTED_VERSION,
    }
}

pub fn wrapping_total(price: u64, quantity: u64) -> u64 {
    price.wrapping_mul(quantity)
}

pub fn checked_total(price: u64, quantity: u64) -> Option<u64> {
    price.checked_mul(quantity)
}

pub fn is_guest_accepted_claim(public_values: &PublicValuesV1) -> bool {
    public_values.chain_id == EXPECTED_CHAIN_ID
        && public_values.context_hash == FIXED_CONTEXT_HASH
        && public_values.version == EXPECTED_VERSION
        && public_values.total == wrapping_total(public_values.price, public_values.quantity)
}

pub fn serialize_public_values(public_values: &PublicValuesV1) -> Vec<u8> {
    let mut out = Vec::with_capacity(88);
    out.extend_from_slice(&public_values.chain_id.to_le_bytes());
    out.extend_from_slice(&public_values.context_hash);
    out.extend_from_slice(&public_values.recipient);
    out.extend_from_slice(&public_values.price.to_le_bytes());
    out.extend_from_slice(&public_values.quantity.to_le_bytes());
    out.extend_from_slice(&public_values.total.to_le_bytes());
    out.extend_from_slice(&public_values.version.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn honest_claim_passes_guest_validation() {
        assert!(is_guest_accepted_claim(&honest_public_values()));
    }

    #[test]
    fn overflowing_claim_passes_guest_validation_with_wrapped_total() {
        let overflow = PublicValuesV1 {
            chain_id: EXPECTED_CHAIN_ID,
            context_hash: FIXED_CONTEXT_HASH,
            recipient: [0x99; 20],
            price: u64::MAX,
            quantity: 2,
            total: wrapping_total(u64::MAX, 2),
            version: EXPECTED_VERSION,
        };

        assert_eq!(overflow.total, u64::MAX - 1);
        assert!(checked_total(overflow.price, overflow.quantity).is_none());
        assert!(is_guest_accepted_claim(&overflow));
    }

    #[test]
    fn wrong_total_is_rejected() {
        let mut wrong_total = honest_public_values();
        wrong_total.total += 1;

        assert!(!is_guest_accepted_claim(&wrong_total));
    }

    #[test]
    fn public_values_layout_matches_the_solidity_decoder() {
        assert_eq!(serialize_public_values(&honest_public_values()).len(), 88);
    }
}
