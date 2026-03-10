# C04 Solution: Integer Overflow In Guest

## Summary

The contract is not the bug. The proof is valid for exactly what the guest proved, and the guest proved the wrong arithmetic rule.

In the starter guest, `total` is checked with wrapping multiplication. An attacker can choose `price` and `quantity` that overflow `u64`, produce a tiny wrapped `total`, and still generate a valid proof.

## Fix strategy

Use checked arithmetic in the guest:

1. Compute `price.checked_mul(quantity)`.
2. Revert if the multiplication overflows.
3. Require the committed `total` to equal the checked product.

This makes overflow impossible to hide inside a valid proof.

## Security takeaway

zk proofs only guarantee faithful execution of the guest program. If the guest uses unsafe arithmetic, the proof will faithfully attest to unsafe arithmetic.
