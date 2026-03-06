#![no_main]
sp1_zkvm::entrypoint!(main);

use common::PublicValuesV1;

pub fn main() {
    let public_values = sp1_zkvm::io::read::<PublicValuesV1>();
    sp1_zkvm::io::commit(&public_values);
}
