// SPDX-License-Identifier: MIT

#![no_main]
sp1_zkvm::entrypoint!(main);

use common::PrivateInputs;
use guest::process_claim;

pub fn main() {
    let inputs = sp1_zkvm::io::read::<PrivateInputs>();
    let claim = process_claim(inputs);
    sp1_zkvm::io::commit(&claim);
}
