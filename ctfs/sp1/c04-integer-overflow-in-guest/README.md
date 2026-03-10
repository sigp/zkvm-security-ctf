# C04 Integer Overflow In Guest

## Goal

Understand why arithmetic inside a zkVM guest must still be range-checked and overflow-safe.

## Scenario

A proof commits a purchase claim with `price`, `quantity`, and `total`. The onchain contract trusts the proved `total` as the required payment and mints `quantity` tokens to the committed recipient.

The bug is in the guest: it validates `total = price * quantity` using wrapping arithmetic, so an overflowing multiplication can still satisfy the guest's policy. That lets an attacker generate a valid proof that mints a large quantity of tokens while paying a near-zero total.

## Difficulty

**Easy** - The contract is fine; the bug lives in the guest arithmetic.

## What you are given

- `starter/`: vulnerable implementation.
  - Host: [starter/rust/host/src/lib.rs](./starter/rust/host/src/lib.rs) builds the guest ELF and can generate proof fixtures.
  - Guest: [starter/rust/guest/src/lib.rs](./starter/rust/guest/src/lib.rs) validates `total` with wrapping multiplication.
  - Common: [starter/rust/common/src/lib.rs](./starter/rust/common/src/lib.rs) defines the purchase claim and helper functions.
  - Verifier: [starter/foundry/src/IntegerOverflowStarter.sol](./starter/foundry/src/IntegerOverflowStarter.sol) verifies proofs and mints tokens based on the committed claim.
- `solution/`: recommended patch that uses checked multiplication in the guest.

## Where to write code

Use this file to complete the exploit after you generate an overflow proof:
- [starter/foundry/test/IntegerOverflowKnowledgeCheck.t.sol](./starter/foundry/test/IntegerOverflowKnowledgeCheck.t.sol)

The exercise scaffold is intentionally not executable yet. Fill in the exploit constants after generating your proof, then rename the function to start with `test`.

## Setup

Build the guest ELF and generate the Solidity fixture library:

```bash
cd ctfs/sp1/c04-integer-overflow-in-guest/starter/rust
cargo prove build
cargo run -p host --release
```

Run starter baseline tests:

```bash
cd ctfs/sp1/c04-integer-overflow-in-guest/starter/foundry
forge test
```

## Exercise workflow

1. Read [starter/rust/common/src/lib.rs](./starter/rust/common/src/lib.rs) and [starter/rust/guest/src/lib.rs](./starter/rust/guest/src/lib.rs).
2. Notice that the guest accepts any claim where `total == wrapping_mul(price, quantity)`.
3. Add an overflowing claim in the starter Rust code, or call `prove_custom()` from the host with your chosen values.
4. Run `cargo run -p host --release` to emit a new fixture.
5. Copy the generated overflow proof and public values into [starter/foundry/test/IntegerOverflowKnowledgeCheck.t.sol](./starter/foundry/test/IntegerOverflowKnowledgeCheck.t.sol).
6. Submit the exploit proof with `msg.value == total` and observe that you receive more tokens than the honest purchase.

## Canonical honest purchase

The starter host emits one honest claim:
- `chain_id = 1`
- `context_hash = 0x2222...2222`
- `recipient = 0x3333...3333`
- `price = 10`
- `quantity = 3`
- `total = 30`
- `version = 1`

## Learning objectives

- Recognize that zkVM guests need safe arithmetic too.
- Understand how overflowing arithmetic can still produce a valid proof if the guest uses the wrong operation.
- Use `checked_mul` and explicit range checks when arithmetic matters to asset accounting.
- Separate "the proof is valid" from "the proved business logic is safe."
