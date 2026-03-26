// SPDX-License-Identifier: MIT

use common::{is_valid_claim, PublicValuesV1, EXPECTED_CHAIN_ID, EXPECTED_VERSION};

pub fn commit_claim(public_values: PublicValuesV1) -> PublicValuesV1 {
    assert!(
        is_valid_claim(&public_values),
        "claim must have chain_id={} and version={}",
        EXPECTED_CHAIN_ID,
        EXPECTED_VERSION
    );

    public_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::honest_public_values;

    #[test]
    fn honest_claim_is_committed() {
        let honest = honest_public_values();
        assert_eq!(commit_claim(honest.clone()), honest);
    }

    #[test]
    fn claim_with_different_recipient_and_amount_is_committed() {
        let modified = PublicValuesV1 {
            chain_id: EXPECTED_CHAIN_ID,
            context_hash: [0xaa; 32],
            recipient: [0x99; 20],
            amount: 999_999,
            nonce: 42,
            version: EXPECTED_VERSION,
        };
        assert_eq!(commit_claim(modified.clone()), modified);
    }

    #[test]
    #[should_panic(expected = "claim must have chain_id=1 and version=1")]
    fn wrong_chain_id_is_rejected() {
        let wrong_chain = PublicValuesV1 {
            chain_id: 999,
            ..honest_public_values()
        };
        let _ = commit_claim(wrong_chain);
    }

    #[test]
    #[should_panic(expected = "claim must have chain_id=1 and version=1")]
    fn wrong_version_is_rejected() {
        let wrong_version = PublicValuesV1 {
            version: 99,
            ..honest_public_values()
        };
        let _ = commit_claim(wrong_version);
    }
}
