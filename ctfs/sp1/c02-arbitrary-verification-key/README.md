# C02 Arbitrary Verification Key

## Goal

Understand how verification keys work.

## Scenario

A contract accepts proof submissions and credits balances using data decoded from the committed public values.
Identify the bug in the contract and exploit it by submitting a malicious proof generated.

## What you are given

- `starter/`: vulnerable implementation.
    - Host: [starter/rust/host/src/main.rs](./starter/rust/host/src/main.rs) - generates fixture values for both the intended program and an attacker-controlled program.
    - Guest: [starter/rust/guest/src/main.rs](./starter/rust/guest/src/main.rs) - intended SP1 program that accepts one canonical payload.
    - Attacker Guest: [starter/rust/attacker/src/main.rs](./starter/rust/attacker/src/main.rs) - malicious SP1 program that commits arbitrary payloads.
    - Verifier: [starter/foundry/src/ArbitraryVerificationKeyStarter.sol](./starter/foundry/src/ArbitraryVerificationKeyStarter.sol) - vulnerable onchain consumer that accepts a caller-supplied program vkey.
- `solution/`: recommended patch.
    - Verifier: [solution/foundry/src/ArbitraryVerificationKeySolution.sol](./solution/foundry/src/ArbitraryVerificationKeySolution.sol) fixed implementation (no peeking!).

## Where to write code

Craft a malicious proof then use this file to test your attack:
- [starter/foundry/test/ArbitraryVerificationKeyKnowledgeCheck.t.sol](./starter/foundry/test/ArbitraryVerificationKeyKnowledgeCheck.t.sol)

It contains an exercise scaffold with TODOs. The test `testExerciseAttackerControlledVKey()` is intentionally failing until you implement the attack.

## Setup

**Optional**: Build the guest ELFs and re-emit fixture values. Fixture values are already included at [fixtures/ArbitraryVerificationKeyFixture.sol](./starter/foundry/test/fixtures/ArbitraryVerificationKeyFixture.sol).

The Foundry exercise uses a verifier harness seeded with the honest and attacker fixture pairs so the focus stays on application-layer verification-key policy.

```bash
cd ctfs/sp1/c02-arbitrary-verification-key/starter/rust
cargo prove build
cargo run -p host --release
```

Run starter baseline tests:
```bash
cd ctfs/sp1/c02-arbitrary-verification-key/starter/foundry
forge test --match-path test/ArbitraryVerificationKeyStarter.t.sol
```

Run the exercise after you fill in the TODOs:
```bash
cd ctfs/sp1/c02-arbitrary-verification-key/starter/foundry
forge test --match-test testExerciseAttackerControlledVKey
```

## Intended program public input envelope

The honest guest only accepts one canonical `PublicValuesV1`:
- `chain_id = 1`
- `context_hash = 0x4444...4444` (32 bytes)
- `recipient = 0x3333...3333`
- `amount = 100`
- `nonce = 1`
- `version = 1`

## Learning objectives

- Distinguish **proof validity** from **program identity**.
- Observe that `verifyProof(vkey, ...)` is only as trustworthy as the `vkey` you pass in.
- Reproduce an exploit where a malicious guest program commits attacker-chosen public values.
- Pin or allowlist expected verification keys in the application layer.
