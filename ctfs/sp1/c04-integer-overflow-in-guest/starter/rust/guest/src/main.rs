// SPDX-License-Identifier: MIT

#![no_main]
sp1_zkvm::entrypoint!(main);

use common::PublicValuesV1;
use guest::commit_claim;

pub fn main() {
    let public_values = sp1_zkvm::io::read::<PublicValuesV1>();
    let public_values = commit_claim(public_values);
    sp1_zkvm::io::commit(&public_values);
}
