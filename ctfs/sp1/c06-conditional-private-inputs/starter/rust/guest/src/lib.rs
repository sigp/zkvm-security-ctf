use common::{is_canonical_claim, ClaimPublicValues, PrivateInputs};

pub fn process_claim(inputs: PrivateInputs) -> ClaimPublicValues {
    if inputs.is_test {
        return inputs.claim;
    }

    assert!(
        is_canonical_claim(&inputs.claim),
        "production proofs must commit the canonical C06 claim"
    );

    inputs.claim
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
    fn test_path_skips_validation_and_commits_the_malicious_claim() {
        let exploit = exploit_private_inputs();
        let committed = process_claim(exploit.clone());
        assert_eq!(committed, exploit.claim);
    }

    #[test]
    #[should_panic(expected = "production proofs must commit the canonical C06 claim")]
    fn production_path_rejects_the_malicious_claim() {
        let mut exploit = exploit_private_inputs();
        exploit.is_test = false;
        let _ = process_claim(exploit);
    }
}
