# zkVM Application Security CTFs

This repository contains hands-on security CTFs for zkVM applications. The current implementation targets SP1 first, with a Risc0 porting roadmap documented separately.

## Purpose And Audience

These CTFs are for Rust and smart contract engineers learning practical zkVM security failure modes:
- insecure proof lifecycle handling,
- insufficient public value constraints,
- key and version management pitfalls,
- witness and arithmetic safety issues.

## Global Prerequisites

- Rust toolchain (`cargo`, `rustc`)
- SP1 tooling (`cargo-prove`, `sp1up`) for full SP1 workflows
- Foundry (`forge`) for Solidity verifier exercises

Setup details: [docs/setup.md](./docs/setup.md)

## Challenge Catalog

| ID | Challenge | Source | Difficulty | Foundry |
|---|---|---|---|---|
| C01 | Replayable Proof Submission | Custom | Easy | Yes |
| C02 | Arbitrary Verification Key | Custom | Easy | Yes |
| C03 | Unconstrained Public Values | Custom | Easy | Yes |
| C04 | Integer Overflow in Guest | Custom | Easy | Yes |
| C05 | Host-Side Validation | Custom | Easy-Medium | Yes |

## Challenges

- [ctfs/sp1/c01-replayable-proof](./ctfs/sp1/c01-replayable-proof)
- [ctfs/sp1/c02-arbitrary-verification-key](./ctfs/sp1/c02-arbitrary-verification-key)
- [ctfs/sp1/c03-unconstrained-public-values](./ctfs/sp1/c03-unconstrained-public-values)
- [ctfs/sp1/c04-integer-overflow-in-guest](./ctfs/sp1/c04-integer-overflow-in-guest)
- [ctfs/sp1/c05-host-side-validation](./ctfs/sp1/c05-host-side-validation)

## How To Use Starter Vs Solution

Each challenge folder has:
- `starter/`: intentionally vulnerable code and tests that demonstrate exploitability.
- `solution/`: fixed implementation and tests that enforce expected security properties.

Recommended flow:
1. Read the challenge README for the scenario you want to work on, for example [C01](./ctfs/sp1/c01-replayable-proof/README.md) or [C02](./ctfs/sp1/c02-arbitrary-verification-key/README.md).
2. Run starter tests and diagnose vulnerable behaviour.
3. Use the starter exercise/tests to reproduce the exploit.
4. Implement the fix in starter yourself.
5. Compare with `solution/`.

## Roadmap

TODO:
- Clean-up Tests
- Make some harder challenges
- Risc0 parity planning for each challenge is in [docs/risc0-roadmap.md](./docs/risc0-roadmap.md).

Considerations of harder examples

- Conditional Private Inputs (Easy)
  - Vulnerability: Guest has multiple execution paths but only validates canonical values on one path
  - Scenario: Guest does if is_test { skip_validation }, attacker triggers test path
  - Learning: All execution paths must be secure, control flow analysis

- Proof Expiration (Medium)
  - Vulnerability: No timestamp/deadline check on proof validity
  - Scenario: Proof generated with old prices/state is submitted after conditions changed
  - Learning: Time-bound proofs, commitment freshness, economic attacks

- Merkle Proof Verification (Medium)
  - Vulnerability: Guest accepts malformed merkle proofs or doesn't verify path ordering
  - Scenario: Prove membership in a set with a forged proof (wrong sibling order, missing nodes)
  - Learning: Merkle tree verification, path validation, leaf vs internal node distinction

- Cross-Contract Proof Sharing (Hard)
  - Vulnerability: Proof intended for Contract A is valid for Contract B (same vkey, different semantics)
  - Scenario: Withdraw proof for Vault A can drain Vault B because both use identical verifier
  - Learning: Domain separation, contract-specific context binding

- State Root Trust (Hard)
  - Vulnerability: Guest trusts a state root from private witness without binding to public commitment
  - Scenario: Attacker provides fake state root proving they own assets they don't have
  - Learning: Binding private inputs to public commitments, storage proofs

- Recursive Proof Aggregation (Hard)
  - Vulnerability: When aggregating multiple proofs, inner proof bindings aren't verified
Batch proof accepts N sub-proofs but doesn't verify they all reference the same context
  - Learning: Proof composition, aggregation security, binding between proofs

- Non-deterministic Proofs (Medium)
  - Vulnerability: Using Groth16 proofs as a nullifier/nonce is not safe as 
  - Scenario: Similar to c01 except we instead as the proof as the nullifier
  - Learning: Proofs are "malleable"

- Private Data In Public Witness (Easy)
  - Vulnerability: Include secret data in the public witness
  - Scenario: We create a signing operations the requires knowledge of some secret key but the key is in the public witness not private
  - Learning: Don't reveal secrets
