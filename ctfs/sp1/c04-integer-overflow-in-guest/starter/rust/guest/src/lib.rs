// SPDX-License-Identifier: MIT

use common::{is_guest_accepted_claim, PublicValuesV1};

pub fn commit_claim(public_values: PublicValuesV1) -> PublicValuesV1 {
    assert!(
        is_guest_accepted_claim(&public_values),
        "guest accepts claims only when total matches wrapping price * quantity"
    );

    public_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{honest_public_values, wrapping_total, EXPECTED_CHAIN_ID, EXPECTED_VERSION, FIXED_CONTEXT_HASH};

    #[test]
    fn honest_claim_is_committed() {
        let honest = honest_public_values();
        assert_eq!(commit_claim(honest.clone()), honest);
    }

    #[test]
    fn overflowing_claim_is_also_committed() {
        let overflow = PublicValuesV1 {
            chain_id: EXPECTED_CHAIN_ID,
            context_hash: FIXED_CONTEXT_HASH,
            recipient: [0x99; 20],
            price: u64::MAX,
            quantity: 2,
            total: wrapping_total(u64::MAX, 2),
            version: EXPECTED_VERSION,
        };

        assert_eq!(commit_claim(overflow.clone()), overflow);
    }

    #[test]
    #[should_panic(expected = "guest accepts claims only when total matches wrapping price * quantity")]
    fn mismatched_total_is_rejected() {
        let mut wrong_total = honest_public_values();
        wrong_total.total = 0;

        let _ = commit_claim(wrong_total);
    }
}
