# C02 Solution: Arbitrary Verification Key

## Summary
The bug is letting the caller choose the verification key at submission time.

In the starter contract, the same verifier contract will happily validate:
- the intended proof with the intended program vkey, and
- an attacker proof with an attacker program vkey.

Because the contract accepts `programVKey` as calldata, it has no policy tying verification to the intended guest image.

## Attack walkthrough
1. The attacker writes a new guest program that commits arbitrary `recipient` and `amount`.
2. The attacker generates a valid proof for that malicious guest, producing a different program vkey but same public input fields.
3. The attacker calls `submit(attackerProgramVKey, attackerProof, attackerPublicValues)`.
4. The onchain contract forwards the attacker-controlled vkey into `verifier.verifyProof(...)`.
5. Proof verification succeeds, and the contract applies the attacker-controlled state transition.

Why this works:
- zk proofs prove a statement relative to a specific circuit / program image.
- They do not prove that the application verified the intended circuit unless the application enforces that policy.

## Root cause
Starter performs:
- `verifier.verifyProof(programVKey, publicValues, proof)`
- state mutation based on decoded public values

but never checks that `programVKey` equals the trusted program image expected by the application.

## Fix strategy
In [solution/foundry/src/ArbitraryVerificationKeySolution.sol](./foundry/src/ArbitraryVerificationKeySolution.sol):
1. Store the trusted program vkey in immutable state.
2. Remove the caller-controlled vkey parameter from `submit`.
3. Always call `verifyProof` with the pinned trusted vkey.

This preserves the intended trust boundary: the verifier proves the public values came from the expected guest program.

## Test behavior
[solution/foundry/test/ArbitraryVerificationKeySolution.t.sol](./foundry/test/ArbitraryVerificationKeySolution.t.sol) asserts:
- the intended proof succeeds under the pinned vkey
- an attacker proof generated from a different guest is rejected
- the attacker receives no balance credit

## Security takeaway
If application security depends on a specific zkVM program, the application must pin or allowlist the expected verification key.

Accepting arbitrary vkeys turns “proof of the intended statement” into “proof of any statement the caller can compile”.
