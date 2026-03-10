// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {IntegerOverflowStarter} from "../src/IntegerOverflowStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {IntegerOverflowFixture} from "./fixtures/IntegerOverflowFixture.sol";

/// @notice Knowledge-check scaffold for C04.
/// @dev This file is intentionally not executed by default because the function name does not start
/// with `test`. Rename the function when you are ready to run it with `forge test`.
contract IntegerOverflowKnowledgeCheck is Test {
    // TODO: Fill in these values after generating an overflow proof with the starter host.
    address constant EXPLOIT_RECIPIENT = address(0); // Your attacker-controlled address
    uint64 constant EXPLOIT_QUANTITY = 0; // Number of tokens you mint
    uint64 constant EXPLOIT_TOTAL = 0; // Wrapped total you will pay onchain
    bytes constant EXPLOIT_PUBLIC_VALUES = ""; // Hex-encoded public values from your proof
    bytes constant EXPLOIT_PROOF = ""; // Hex-encoded proof bytes

    function exerciseOverflowMintForCheap() public {
        // Skip if exploit values haven't been filled in
        vm.assume(EXPLOIT_RECIPIENT != address(0));
        vm.assume(EXPLOIT_QUANTITY > IntegerOverflowFixture.HONEST_QUANTITY);
        vm.assume(EXPLOIT_PUBLIC_VALUES.length == 88);
        vm.assume(EXPLOIT_PROOF.length > 0);

        SP1Verifier verifier = new SP1Verifier();
        IntegerOverflowStarter c =
            new IntegerOverflowStarter(address(verifier), IntegerOverflowFixture.PROGRAM_VKEY);

        c.submit{value: IntegerOverflowFixture.HONEST_TOTAL}(
            IntegerOverflowFixture.HONEST_PROOF, IntegerOverflowFixture.HONEST_PUBLIC_VALUES
        );

        // ============================== Begin Exercise ==============================================
        // TODO: Submit your overflow proof while paying only EXPLOIT_TOTAL.


        // ============================== End Exercise ================================================

        assertEq(
            c.tokenBalances(IntegerOverflowFixture.HONEST_RECIPIENT),
            uint256(IntegerOverflowFixture.HONEST_QUANTITY),
            "baseline honest submit failed"
        );

        assertEq(c.tokenBalances(EXPLOIT_RECIPIENT), uint256(EXPLOIT_QUANTITY), "exploit failed");
        assertLt(EXPLOIT_TOTAL, IntegerOverflowFixture.HONEST_TOTAL, "exploit should be cheaper than the honest mint");
    }
}
