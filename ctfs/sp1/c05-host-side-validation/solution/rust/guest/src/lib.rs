// SPDX-License-Identifier: MIT

use common::{is_honest_claim, PublicValuesV1};

pub fn commit_claim(public_values: PublicValuesV1) -> PublicValuesV1 {
    assert!(
        is_honest_claim(&public_values),
        "guest only accepts the canonical C05 claim"
    );

    public_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{honest_public_values, PublicValuesV1};

    #[test]
    fn honest_claim_is_committed() {
        let honest = honest_public_values();
        assert_eq!(commit_claim(honest.clone()), honest);
    }

    #[test]
    #[should_panic(expected = "guest only accepts the canonical C05 claim")]
    fn attacker_claim_is_rejected() {
        let attacker = PublicValuesV1 {
            chain_id: 1,
            context_hash: [0xaa; 32],
            recipient: [0x99; 20],
            amount: 1_000_000,
            nonce: 4_242,
            version: 1,
        };

        let _ = commit_claim(attacker);
    }
}
