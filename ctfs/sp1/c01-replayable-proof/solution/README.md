# C01 Solution: Replayable Proof Submission

## Summary
The bug is **not** invalid proving. The bug is missing **consumption tracking** for valid proofs.

In the starter contract, a valid `(proof, publicValues)` pair can be submitted repeatedly, and each submission applies state changes again.

The fix is to add deterministic replay protection over committed values and reject repeated use.

## Attack walkthrough
1. Attacker obtains any valid proof payload accepted by the contract.
2. Calls `submit(proof, publicValues)` once; contract verifies proof and applies effect.
3. Calls `submit(proof, publicValues)` again with exactly the same bytes.
4. Proof verification still passes, so effect is applied again.

Why this works:
- zk proof systems establish statement validity.
- They do **not** enforce one-time usage semantics at the application layer.

## Root cause
Starter performs:
- `verifier.verifyProof(...)`
- state mutation (credit)

but never records that this exact committed statement has already been consumed.

## Fix strategy
In `solution/foundry/src/ReplayableProofSolution.sol`:
1. Derive a nullifier from committed public values using domain separation.
2. Track nullifier usage in storage.
3. Revert if nullifier already consumed.
4. Only apply effect when proof is valid and nullifier is fresh.

This binds replay prevention to proof-committed data rather than caller identity.

## Foundry implementation details
The solution Foundry setup now matches the starter’s real verifier plumbing:
- Real verifier interface import: `@sp1-contracts/ISP1Verifier.sol`
- Real verifier deployment in tests: `@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol`
- Real fixture values:
  - `solution/foundry/test/fixtures/ReplayableProofSolutionFixture.sol`

## Test behavior
`solution/foundry/test/ReplayableProofSolution.t.sol` asserts:
- first valid submission succeeds
- second identical submission reverts (`Replay` path)
- credited balance remains single-credit only

## Security takeaway
Whenever proofs authorize side effects, you need both:
- cryptographic validity checks, and
- protocol-level single-use/replay controls.

Without both, valid proofs can still be economically exploitable.
