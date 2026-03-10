use common::{is_canonical_payload_except_chain_id, PublicValuesV1};

pub fn validate_public_values(public_values: PublicValuesV1) -> PublicValuesV1 {
    assert!(
        is_canonical_payload_except_chain_id(&public_values),
        "guest only accepts the canonical C03 payload with a caller-supplied chain_id"
    );

    public_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{foreign_public_values, honest_public_values};

    #[test]
    fn canonical_payload_is_accepted_for_multiple_chain_ids() {
        let honest = honest_public_values();
        let foreign = foreign_public_values();

        assert_eq!(validate_public_values(honest.clone()), honest);
        assert_eq!(validate_public_values(foreign.clone()), foreign);
    }

    #[test]
    #[should_panic(
        expected = "guest only accepts the canonical C03 payload with a caller-supplied chain_id"
    )]
    fn tampered_amount_is_rejected() {
        let mut public_values = honest_public_values();
        public_values.amount += 1;

        let _ = validate_public_values(public_values);
    }
}
