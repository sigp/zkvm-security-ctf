// SPDX-License-Identifier: MIT

#![no_main]
sp1_zkvm::entrypoint!(main);

use common::PublicValuesV1;
use guest::validate_public_values;

pub fn main() {
    let public_values = sp1_zkvm::io::read::<PublicValuesV1>();
    let public_values = validate_public_values(public_values);
    sp1_zkvm::io::commit(&public_values);
}
