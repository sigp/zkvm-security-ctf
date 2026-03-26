use common::{is_canonical_claim, ClaimPublicValues, PrivateInputs};

pub fn process_claim(inputs: PrivateInputs) -> ClaimPublicValues {
    let claim = inputs.claim;

    assert!(
        is_canonical_claim(&claim),
        "all proofs must commit the canonical C06 claim"
    );

    claim
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{canonical_public_values, exploit_private_inputs, honest_private_inputs};

    #[test]
    fn production_path_accepts_the_canonical_claim() {
        let honest = honest_private_inputs();
        assert_eq!(process_claim(honest), canonical_public_values());
    }

    #[test]
    #[should_panic(expected = "all proofs must commit the canonical C06 claim")]
    fn test_path_no_longer_skips_validation() {
        let _ = process_claim(exploit_private_inputs());
    }
}
