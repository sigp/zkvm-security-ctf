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

pub fn fixed_public_values() -> PublicValuesV1 {
    PublicValuesV1 {
        chain_id: FIXED_CHAIN_ID,
        context_hash: FIXED_CONTEXT_HASH,
        recipient: FIXED_RECIPIENT,
        amount: FIXED_AMOUNT,
        nonce: FIXED_NONCE,
        version: FIXED_VERSION,
    }
}

pub fn is_fixed_solution(public_values: &PublicValuesV1) -> bool {
    public_values == &fixed_public_values()
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

pub fn hash_public_values(public_values: &PublicValuesV1) -> [u8; 32] {
    hash_bytes(&serialize_public_values(public_values))
}

pub fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut state = [0u8; 32];

    for (i, byte) in data.iter().enumerate() {
        let idx = i % 32;
        let rot = (i % 8) as u32;
        state[idx] = state[idx].wrapping_add(byte.rotate_left(rot));
        state[(idx + 7) % 32] ^= byte.wrapping_mul(31);
        state[(idx + 13) % 32] = state[(idx + 13) % 32].wrapping_add((i as u8).wrapping_mul(17));
    }

    for (i, slot) in state.iter_mut().enumerate() {
        *slot = slot.wrapping_add((i as u8).wrapping_mul(19));
    }

    state
}
