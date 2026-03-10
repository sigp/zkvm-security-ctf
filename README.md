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
| C02 | Arbitrary Verification Key | Custom | Easy-Med | Yes |
| C03 | Unconstrained Public Values | Custom | Easy-Med | Yes |
| C05 | Host-Side Validation | Custom | Medium | Yes |

## How To Use Starter Vs Solution

Each challenge folder has:
- `starter/`: intentionally vulnerable code and tests that demonstrate exploitability.
- `solution/`: fixed implementation and tests that enforce expected security properties.

Recommended flow:
1. Read the challenge README for the scenario you want to work on, for example [C01](./ctfs/sp1/c01-replayable-proof/README.md), [C02](./ctfs/sp1/c02-arbitrary-verification-key/README.md), [C03](./ctfs/sp1/c03-unconstrained-public-values/README.md), or [C05](./ctfs/sp1/c05-host-side-validation/README.md).
2. Run starter tests and diagnose vulnerable behaviour.
3. Use the starter exercise/tests to reproduce the exploit.
4. Implement the fix in starter yourself.
5. Compare with `solution/`.

## Risc0 Roadmap

Risc0 parity planning for each challenge is in [docs/risc0-roadmap.md](./docs/risc0-roadmap.md).

## Challenges

- [ctfs/sp1/c01-replayable-proof](./ctfs/sp1/c01-replayable-proof)
- [ctfs/sp1/c02-arbitrary-verification-key](./ctfs/sp1/c02-arbitrary-verification-key)
- [ctfs/sp1/c03-unconstrained-public-values](./ctfs/sp1/c03-unconstrained-public-values)
- [ctfs/sp1/c05-host-side-validation](./ctfs/sp1/c05-host-side-validation)
