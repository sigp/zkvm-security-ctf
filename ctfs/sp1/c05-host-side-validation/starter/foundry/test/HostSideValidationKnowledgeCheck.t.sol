// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {HostSideValidationStarter} from "../src/HostSideValidationStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {HostSideValidationFixture} from "./fixtures/HostSideValidationFixture.sol";

/// @notice Knowledge-check scaffold for C05.
/// @dev This file is intentionally not executed by default because the function name does not start
/// with `test`. Rename the function when you are ready to run it with `forge test`.
///
/// EXERCISE:
/// 1. Analyze the guest program (starter/rust/guest/src/lib.rs) to identify which fields
///    are validated and which are unconstrained.
/// 2. Modify the common lib or create a custom proof by calling `prove_custom()` from the host.
/// 3. Generate a proof that credits a different recipient with a different amount.
/// 4. Fill in the constants below with your exploit values and run the test.
///
/// HINTS:
/// - The guest validates some fields but not others. Read commit_claim() carefully.
/// - Use `cargo run -p host` after modifying the code to generate new proof fixtures.
/// - The public values are 80 bytes: chain_id(8) + context_hash(32) + recipient(20) + amount(8) + nonce(8) + version(4)
contract HostSideValidationKnowledgeCheck is Test {
    // TODO: Fill in your exploit values after generating a malicious proof
    address constant EXPLOIT_RECIPIENT = address(0); // Your attacker-controlled address
    uint64 constant EXPLOIT_AMOUNT = 0; // Amount you want to credit
    bytes constant EXPLOIT_PUBLIC_VALUES = ""; // Hex-encoded public values from your proof
    bytes constant EXPLOIT_PROOF = ""; // Hex-encoded proof bytes

    function exerciseExploitUnconstrained() public {
        // Skip if exploit values haven't been filled in
        vm.assume(EXPLOIT_RECIPIENT != address(0));
        vm.assume(EXPLOIT_AMOUNT > 0);
        vm.assume(EXPLOIT_PUBLIC_VALUES.length == 80);
        vm.assume(EXPLOIT_PROOF.length > 0);

        SP1Verifier verifier = new SP1Verifier();
        HostSideValidationStarter c =
            new HostSideValidationStarter(address(verifier), HostSideValidationFixture.PROGRAM_VKEY);

        // Submit the honest proof first
        c.submit(HostSideValidationFixture.HONEST_PROOF, HostSideValidationFixture.HONEST_PUBLIC_VALUES);

        // ============================== Begin Exercise ==============================================
        // TODO: Submit your exploit proof that credits EXPLOIT_RECIPIENT with EXPLOIT_AMOUNT.
        // The proof must be valid under the same PROGRAM_VKEY but with different public values.


        // ============================== End Exercise ================================================

        // Verify honest submission worked
        assertEq(
            c.balances(HostSideValidationFixture.HONEST_RECIPIENT),
            uint256(HostSideValidationFixture.HONEST_AMOUNT),
            "baseline honest submit failed"
        );

        // Verify exploit worked - your proof credited a different recipient
        assertEq(c.balances(EXPLOIT_RECIPIENT), uint256(EXPLOIT_AMOUNT), "exploit failed");

        // Bonus: verify you got more than the honest amount
        assertGt(EXPLOIT_AMOUNT, HostSideValidationFixture.HONEST_AMOUNT, "exploit amount should exceed honest amount");
    }
}
