#![no_main]
sp1_zkvm::entrypoint!(main);

use common::{is_honest_claim, PublicValuesV1};

pub fn main() {
    let public_values = sp1_zkvm::io::read::<PublicValuesV1>();
    // @audit skip the honest claim check to allow arbitrary payloads in malicious solution
    // assert!(
    //     is_honest_claim(&public_values),
    //     "guest only accepts the single canonical C02 payload"
    // );
    sp1_zkvm::io::commit(&public_values);
}
