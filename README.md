# zkVM Security CTFs (SP1 v1)

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

Setup details: [docs/setup.md](/home/kirk/work/audits/zkvm-security-ctf/docs/setup.md)

## Challenge Catalog

| ID | Challenge | Source | Difficulty | Foundry |
|---|---|---|---|---|
| C01 | Replayable Proof Submission | Custom | Easy | Yes |
| C02 | Trusted Private Witness Data | Custom | Easy-Med | No |
| C03 | Unconstrained Public Values In Verifier Flow | Custom | Medium | Yes |
| C04 | Guest Arithmetic + Public Input Aggregation Overflow | Custom | Medium | No |
| C05 | Missing Public Value Commitments | Blog-derived | Medium | No |
| C06 | Program Version / Image ID Management | Blog-derived | Med-Hard | Yes |
| C07 | Verification Key Management | Blog-derived | Hard | Yes |
| C08 | Precompile Witness Constraint Gaps | Blog-derived | Hard | No |

## How To Use Starter Vs Solution

Each challenge folder has:
- `starter/`: intentionally vulnerable code and tests that demonstrate exploitability.
- `solution/`: fixed implementation and tests that enforce expected security properties.

Recommended flow:
1. Read the challenge `README.md`.
2. Run starter tests to observe vulnerable behavior.
3. Implement the fix in starter yourself.
4. Compare with `solution/`.

## Integrity Notice

Reference solutions are intentionally included in-repo under `solution/` so this can be used as a guided training curriculum rather than a blind competition environment.

## Risc0 Roadmap

Risc0 parity planning for each challenge is in [docs/risc0-roadmap.md](/home/kirk/work/audits/zkvm-security-ctf/docs/risc0-roadmap.md).

## Challenges

- [ctfs/sp1/c01-replayable-proof](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c01-replayable-proof)
- [ctfs/sp1/c02-trusted-private-witness](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c02-trusted-private-witness)
- [ctfs/sp1/c03-unconstrained-public-values](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c03-unconstrained-public-values)
- [ctfs/sp1/c04-overflow-constraints](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c04-overflow-constraints)
- [ctfs/sp1/c05-missing-public-commitments](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c05-missing-public-commitments)
- [ctfs/sp1/c06-program-version-management](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c06-program-version-management)
- [ctfs/sp1/c07-verification-key-management](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c07-verification-key-management)
- [ctfs/sp1/c08-precompile-witness-constraints](/home/kirk/work/audits/zkvm-security-ctf/ctfs/sp1/c08-precompile-witness-constraints)
