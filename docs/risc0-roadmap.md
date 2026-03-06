# Risc0 Porting Roadmap (Planned)

Status in this document is planning only. SP1 is implemented first.

| ID | Risc0 Equivalent Concept | Guest I/O + API Mapping Notes | Verifier / Contract Adaptation | Porting Risks / Expected Diffs | Status |
|---|---|---|---|---|---|
| C01 | Replay nullifier enforcement in proof submission | Map SP1 public values serialization to Risc0 journal encoding | Keep `submit(proof, publicValues)` shape and preserve nullifier derivation | Journal encoding differences may affect hash preimages | Planned |
| C02 | Private witness authenticity binding | Replace SP1 witness handling with Risc0 guest input + authenticated envelope | Mostly host-side; no Solidity requirement | Signature parsing and guest input ABI differences | Planned |
| C03 | Constrained public values before state effects | Enforce identical full-field constraints over journal commitments | Preserve verifier-side prechecks and decode logic | Risk of partial-commit regressions during port | Planned |
| C04 | Overflow-safe arithmetic constraints in guest/host | Keep overflow checks in host logic and bound fields in guest outputs | No contract dependency | Different integer helpers/serialization conventions | Planned |
| C05 | Full commitment of security-relevant public values | Commit all fields in Risc0 journal hash domain | Host must validate all fields before apply | Omitting fields when converting commitment helpers | Planned |
| C06 | Program/image version allowlisting | Map SP1 program-id concept to Risc0 image-id policy | Solidity verifier should gate accepted image IDs/versions | Version registry model may need Risc0-specific metadata | Planned |
| C07 | Verification key governance and rotation safety | Keep key-id binding in host verification metadata | Contract key governance unchanged; proof system adapter may differ | Activation semantics can drift without strict tests | Planned |
| C08 | Precompile witness semantic constraints | Port witness semantics checks and domain separation to Risc0-compatible APIs | No contract dependency | Crypto helper behavior differences can hide constraint gaps | Planned |
