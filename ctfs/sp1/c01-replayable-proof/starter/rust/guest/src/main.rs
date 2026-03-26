// SPDX-License-Identifier: MIT

#![no_main]
sp1_zkvm::entrypoint!(main);

use common::{is_fixed_solution, PublicValuesV1};

pub fn main() {
    let public_values = sp1_zkvm::io::read::<PublicValuesV1>();
    assert!(
        is_fixed_solution(&public_values),
        "guest only accepts the single canonical C01 payload"
    );
    sp1_zkvm::io::commit(&public_values);
}
