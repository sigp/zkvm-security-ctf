use common::{checked_total, PublicValuesV1};

pub fn commit_claim(public_values: PublicValuesV1) -> PublicValuesV1 {
    let expected_total = checked_total(public_values.price, public_values.quantity)
        .expect("price * quantity must not overflow");
    assert!(
        public_values.total == expected_total,
        "guest requires total == checked price * quantity"
    );

    public_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{honest_public_values, PublicValuesV1, EXPECTED_CHAIN_ID, EXPECTED_VERSION, FIXED_CONTEXT_HASH};

    #[test]
    fn honest_claim_is_committed() {
        let honest = honest_public_values();
        assert_eq!(commit_claim(honest.clone()), honest);
    }

    #[test]
    #[should_panic(expected = "price * quantity must not overflow")]
    fn overflowing_claim_is_rejected() {
        let attacker = PublicValuesV1 {
            chain_id: EXPECTED_CHAIN_ID,
            context_hash: FIXED_CONTEXT_HASH,
            recipient: [0x99; 20],
            price: u64::MAX,
            quantity: 2,
            total: u64::MAX - 1,
            version: EXPECTED_VERSION,
        };

        let _ = commit_claim(attacker);
    }
}
