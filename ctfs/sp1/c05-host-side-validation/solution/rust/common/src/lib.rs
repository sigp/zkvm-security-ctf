use serde::{Deserialize, Serialize};

pub const FIXED_CHAIN_ID: u64 = 1;
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
    PublicValuesV1 {
        chain_id: FIXED_CHAIN_ID,
        context_hash: FIXED_CONTEXT_HASH,
        recipient: FIXED_RECIPIENT,
        amount: FIXED_AMOUNT,
        nonce: FIXED_NONCE,
        version: FIXED_VERSION,
    }
}

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
    fn honest_claim_matches_the_fixed_fixture() {
        assert!(is_honest_claim(&honest_public_values()));
    }

    #[test]
    fn public_values_layout_matches_the_solidity_decoder() {
        assert_eq!(serialize_public_values(&honest_public_values()).len(), 80);
    }
}
