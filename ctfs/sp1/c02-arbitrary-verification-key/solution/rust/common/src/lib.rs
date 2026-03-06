use serde::{Deserialize, Serialize};

pub const HONEST_CHAIN_ID: u64 = 1;
pub const HONEST_CONTEXT_HASH: [u8; 32] = [0x44; 32];
pub const HONEST_RECIPIENT: [u8; 20] = [0x33; 20];
pub const HONEST_AMOUNT: u64 = 100;
pub const HONEST_NONCE: u64 = 1;
pub const HONEST_VERSION: u32 = 1;

pub const ATTACKER_CHAIN_ID: u64 = 31_337;
pub const ATTACKER_CONTEXT_HASH: [u8; 32] = [0xaa; 32];
pub const ATTACKER_RECIPIENT: [u8; 20] = [0x99; 20];
pub const ATTACKER_AMOUNT: u64 = 1_000_000;
pub const ATTACKER_NONCE: u64 = 4_242;
pub const ATTACKER_VERSION: u32 = 2;

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
        chain_id: HONEST_CHAIN_ID,
        context_hash: HONEST_CONTEXT_HASH,
        recipient: HONEST_RECIPIENT,
        amount: HONEST_AMOUNT,
        nonce: HONEST_NONCE,
        version: HONEST_VERSION,
    }
}

pub fn attacker_public_values() -> PublicValuesV1 {
    PublicValuesV1 {
        chain_id: ATTACKER_CHAIN_ID,
        context_hash: ATTACKER_CONTEXT_HASH,
        recipient: ATTACKER_RECIPIENT,
        amount: ATTACKER_AMOUNT,
        nonce: ATTACKER_NONCE,
        version: ATTACKER_VERSION,
    }
}

pub fn is_honest_claim(public_values: &PublicValuesV1) -> bool {
    public_values == &honest_public_values()
}
