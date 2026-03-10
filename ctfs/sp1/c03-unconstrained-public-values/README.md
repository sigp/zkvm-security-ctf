# C03 Unconstrained Public Values

## Goal

Understand why some public values must be constrained in the verifier, not just committed by the proof.

## Scenario

A valid SP1 proof commits a full claim payload, including `chain_id`, but the guest does not know which chain this proof is for because the guest is not a chain, it's offchain code.
Identify the bug in the contract and exploit it with a proof that was generated for a different chain.

## What you are given

- `starter/`: vulnerable implementation.
    - Host: [starter/rust/host/src/main.rs](./starter/rust/host/src/main.rs) - generates two Groth16 fixtures for the same guest program: one for the honest chain and one for a foreign chain.
    - Guest: [starter/rust/guest/src/main.rs](./starter/rust/guest/src/main.rs) - SP1 program that commits the full `PublicValuesV1`, but only constrains the canonical payload *except* for `chain_id`.
    - Verifier: [starter/foundry/src/UnconstrainedVerifierStarter.sol](./starter/foundry/src/UnconstrainedVerifierStarter.sol) - vulnerable onchain consumer that verifies proofs and prevents exact replay, but forgets to check that the committed `chain_id` matches `block.chainid`.
- `solution/`: recommended patch.
    - Verifier: [solution/foundry/src/UnconstrainedVerifierSolution.sol](./solution/foundry/src/UnconstrainedVerifierSolution.sol) fixed implementation (no peeking!).

## Where to write code

Use this file to implement the exploit:
- [starter/foundry/test/UnconstrainedVerifierKnowledgeCheck.t.sol](./starter/foundry/test/UnconstrainedVerifierKnowledgeCheck.t.sol)

It contains an exercise scaffold with TODOs. The function `exerciseForeignChainProofIsAccepted()` is not executed by default; rename it to start with `test` when you are ready to run the exercise.

## Setup

**Optional**: build the guest ELF and regenerate the Solidity fixture library. Fixture values are written into both starter and solution at `test/fixtures/UnconstrainedVerifierFixture.sol`.

```bash
cd ctfs/sp1/c03-unconstrained-public-values/starter/rust
cargo prove build
cargo run -p host --release
```

Run starter baseline tests:
```bash
cd ctfs/sp1/c03-unconstrained-public-values/starter/foundry
forge test
```

Run the exercise after filling in the TODO and renaming the function to start with `test`:
```bash
cd ctfs/sp1/c03-unconstrained-public-values/starter/foundry
forge test --match-test testExerciseForeignChainProofIsAccepted
```

Run solution tests:
```bash
cd ctfs/sp1/c03-unconstrained-public-values/solution/foundry
forge test
```

## Canonical SP1 public input envelope

The guest accepts exactly one canonical `PublicValuesV1` envelope for every field except `chain_id`:
- `chain_id = caller supplied`
- `context_hash = 0x2222...2222` (32 bytes)
- `recipient = 0x3333...3333`
- `amount = 100`
- `nonce = 1`
- `version = 1`

The host emits two valid proofs under the same program key:
- `HONEST_CHAIN_ID = 1`
- `FOREIGN_CHAIN_ID = 10`

## Learning objectives

- Distinguish **proof validity** from **application-level chain binding**.
- Observe that committing `chain_id` is not enough if the verifier never compares it to `block.chainid`.
- Reproduce an exploit where a proof intended for chain `10` is accepted on chain `1`.
- Constrain verifier-visible public values before applying any state effect.
