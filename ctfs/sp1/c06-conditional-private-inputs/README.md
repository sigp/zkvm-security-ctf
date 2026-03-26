# C06 Conditional Private Inputs

## Goal

Understand why security checks in a zkVM guest must hold across every execution path, including branches driven by private witness data.

## Scenario

An SP1 guest reads a private `is_test` flag alongside a claim. In production mode, it enforces one canonical claim. In test mode, it skips validation and still commits the claim as public values.

The Rust verifier is not buggy: it verifies proofs under one trusted program key and credits the committed recipient and amount. The problem is that the verifier cannot see the private `is_test` flag, so an attacker can generate a valid proof for a malicious claim by taking the insecure test path.

## Difficulty

**Easy** - The verifier is fine. The bug is a control-flow issue in the guest.

## What you are given

- `starter/`: vulnerable Rust-only implementation.
  - Common: [starter/rust/common/src/lib.rs](./starter/rust/common/src/lib.rs) defines the public claim, the private witness, and the canonical values.
  - Guest: [starter/rust/guest/src/lib.rs](./starter/rust/guest/src/lib.rs) has the vulnerable `if is_test { ... }` branch.
  - Host: [starter/rust/host/src/lib.rs](./starter/rust/host/src/lib.rs) generates fast SP1 core proofs for arbitrary private inputs.
  - Verifier: [starter/rust/verifier/src/lib.rs](./starter/rust/verifier/src/lib.rs) verifies proofs in Rust and applies the committed balance update.
- `solution/`: recommended patch where every path enforces the same claim policy.

## Where to write code

Start by reading:
- [starter/rust/common/src/lib.rs](./starter/rust/common/src/lib.rs)
- [starter/rust/guest/src/lib.rs](./starter/rust/guest/src/lib.rs)
- [starter/rust/verifier/src/lib.rs](./starter/rust/verifier/src/lib.rs)

Then complete the exercise scaffold in:
- [starter/rust/verifier/src/lib.rs](./starter/rust/verifier/src/lib.rs)

The test `exercise_test_mode_skips_validation` is intentionally ignored. Remove the `#[ignore]` and fill in the TODOs when you are ready to run your exploit.

## Setup

This challenge is Rust-only and uses native SP1 verification with fast mock proofs. The guest still executes, the verifier still pins one trusted program key, and the host crate auto-builds the guest ELF for you, so you do not need to run `cargo prove build` manually.

Run starter tests:

```bash
cd ctfs/sp1/c06-conditional-private-inputs/starter/rust
cargo test
```

Run the exercise after completing the ignored test:

```bash
cd ctfs/sp1/c06-conditional-private-inputs/starter/rust
cargo test -p verifier exercise_test_mode_skips_validation -- --ignored
```

Run solution tests:

```bash
cd ctfs/sp1/c06-conditional-private-inputs/solution/rust
cargo test
```

## Canonical Claim

The intended production proof commits exactly one canonical `ClaimPublicValues`:
- `chain_id = 1`
- `context_hash = 0x2222...2222` (32 bytes)
- `recipient = 0x3333...3333`
- `amount = 100`
- `nonce = 1`
- `version = 1`

The private `is_test` flag is not committed as a public value.

## Learning Objectives

- Identify security-critical guest branches that depend on private inputs.
- Understand that the verifier only sees committed public values, not the private path used to produce them.
- Reproduce an exploit where a valid proof is generated from an insecure guest branch.
- Apply the right fix: enforce the same security policy on every path, or remove the unsafe branch entirely.
