# C01 Replayable Proof Submission

## Goal

Understand why a valid zk proof can still be unsafe to consume repeatedly.

## Scenario

A contract accepts SP1 Groth16 proofs and applies a state effect (credits a balance).
Identify the bug in the contract and exploit it to double your balance.

## What you are given

- `starter/`: vulnerable implementation.
    - Host: [starter/rust/host/src/main.rs](./starter/rust/host/src/main.rs) - generates the SP1 Groth16 proof, and emits fixture values.
    - Guest: [starter/rust/guest/src/main.rs](./starter/rust/guest/src/main.rs) - SP1 program that accepts one canonical payload and commits it as public values.
    - Verifier: [starter/foundry/src/ReplayableProofStarter.sol](./starter/foundry/src/ReplayableProofStarter.sol) - vulnerable onchain consumer that verifies proofs.
- `solution/`: recommended patch.
    - Verifier: [solution/foundry/src/ReplayableProofSolution.sol](./solution/foundry/src/ReplayableProofSolution.sol) fixed implementation (no peeking!).

## Where to write code

This scenario is a **Verifier** only challenge, you do not need to modify the Guest or Host code.

Use this file to implement your attack:
- [starter/foundry/test/ReplayableProofKnowledgeCheck.t.sol](./starter/foundry/test/ReplayableProofKnowledgeCheck.t.sol)

It contains an exercise scaffold with TODOs. The test `testExerciseReplayAttack()` is currently failing, modify the test such that it passes.

## Setup

**Optional**: Build and prove (host) which will take 15-30 mins. Fixture values are already included at [fixtures/ReplayableProofStarterFixture.sol](./starter/foundry/test/fixtures/ReplayableProofStarterFixture.sol).

```bash
cd ctfs/sp1/c01-replayable-proof/starter/rust
cargo run -p host --release
```

Run starter tests:
```bash
cd ctfs/sp1/c01-replayable-proof/starter/foundry
forge test
```

## Canonical SP1 public input envelope

The guest only accepts one canonical `PublicValuesV1`:
- `chain_id = 1`
- `context_hash = 0x2222...2222` (32 bytes)
- `recipient = 0x3333...3333`
- `amount = 100`
- `nonce = 1`
- `version = 1`

## Learning objectives

- Distinguish **proof validity** from **proof single-use semantics**.
- Reproduce a replay attack using an identical `(proof, publicValues)` pair.
- Design replay protection tied to committed proof context.
