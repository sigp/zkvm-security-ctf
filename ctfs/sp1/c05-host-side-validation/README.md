# C05 Host-Side Validation

## Goal

Understand why checks in the proving host do not automatically become part of the proved statement, even when the guest validates some fields.

## Scenario

An application trusts one SP1 program key and credits balances from the proof's committed public values. The guest program validates `chain_id` and `version`, but are all fields constrained?

Your task:
1. Analyze the guest program to identify which fields are validated vs. unconstrained.
2. Generate a valid proof with malicious public values.
3. Submit the exploit proof to credit an attacker-controlled recipient.

## Difficulty

**Medium** - Requires analyzing Rust guest code and generating custom proofs.

## What you are given

- `starter/`: vulnerable implementation.
  - Host: [starter/rust/host/src/lib.rs](./starter/rust/host/src/lib.rs) builds the guest ELF and generates proof fixtures.
  - Guest: [starter/rust/guest/src/lib.rs](./starter/rust/guest/src/lib.rs) validates some fields before committing.
  - Common: [starter/rust/common/src/lib.rs](./starter/rust/common/src/lib.rs) defines the `PublicValuesV1` struct and validation logic.
  - Verifier: [starter/foundry/src/HostSideValidationStarter.sol](./starter/foundry/src/HostSideValidationStarter.sol) verifies proofs under a fixed program key and applies the committed claim.
- `solution/`: recommended patch that constrains all fields in the guest.

## Where to write code

1. Analyze the guest to identify unconstrained fields:
   - [starter/rust/guest/src/lib.rs](./starter/rust/guest/src/lib.rs)
   - [starter/rust/common/src/lib.rs](./starter/rust/common/src/lib.rs)

2. Modify the common lib to define your exploit claim, then regenerate the proof:
   - Edit `common/src/lib.rs` to add your malicious public values
   - Run `cargo run -p host --release` to generate a new fixture

3. Complete the exercise in:
   - [starter/foundry/test/HostSideValidationKnowledgeCheck.t.sol](./starter/foundry/test/HostSideValidationKnowledgeCheck.t.sol)

## Setup

Build the guest ELF and generate the Solidity fixture library:

```bash
cd ctfs/sp1/c05-host-side-validation/starter/rust
cargo prove build
cargo run -p host --release
```

Run starter baseline tests:
```bash
cd ctfs/sp1/c05-host-side-validation/starter/foundry
forge test
```

## Exercise workflow

1. **Analyze the guest** - Read `commit_claim()` in `guest/src/lib.rs`. What does it check? What doesn't it check?

2. **Identify exploitable fields** - The `PublicValuesV1` struct has 6 fields:
   - `chain_id` (8 bytes)
   - `context_hash` (32 bytes)
   - `recipient` (20 bytes)
   - `amount` (8 bytes)
   - `nonce` (8 bytes)
   - `version` (4 bytes)

   Which fields can you modify while still generating a valid proof?

3. **Generate an exploit proof** - Modify `common/src/lib.rs` to add a function that returns your malicious public values, then update `host/src/lib.rs` to call `prove_custom()` with your values:

   ```rust
   // Example: In common/src/lib.rs, add:
   pub fn exploit_public_values() -> PublicValuesV1 {
       PublicValuesV1 {
           chain_id: EXPECTED_CHAIN_ID,  // Must match
           version: EXPECTED_VERSION,     // Must match
           // But these can be anything:
           recipient: [0x99; 20],
           amount: 1_000_000,
           context_hash: [0xaa; 32],
           nonce: 42,
       }
   }
   ```

4. **Run the host** to generate your exploit fixture:
   ```bash
   cargo run -p host --release
   ```

5. **Fill in the knowledge check** - Copy your exploit values into `HostSideValidationKnowledgeCheck.t.sol` and rename the function to start with `test`.

6. **Run the exploit**:
   ```bash
   cd ../foundry
   forge test --match-test testExercise
   ```

## Solution

Run solution tests:
```bash
cd ctfs/sp1/c05-host-side-validation/solution/rust
cargo prove build
cargo run -p host --release
cargo test --workspace
cd ../foundry
forge test
```

The solution moves validation of ALL fields into the guest program, making it impossible to generate valid proofs with unauthorized values.

## Learning objectives

- Analyze guest code to identify which fields are constrained vs. unconstrained.
- Understand that partial validation in the guest still leaves attack surface.
- Generate custom proofs by modifying the proving pipeline.
- Recognize that a trusted program key only guarantees correct execution, not policy compliance.
- Move security checks into the proved statement to make them cryptographically enforced.
