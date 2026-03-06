// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ReplayableProofStarter} from "../src/ReplayableProofStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {ReplayableProofStarterFixture} from "./fixtures/ReplayableProofStarterFixture.sol";

/// @notice Knowledge-check scaffold for C01.
/// @dev This file is intentionally not executed by default because function names do not start with
/// `test`. Rename the function when you are ready to run it with `forge test`.
contract ReplayableProofKnowledgeCheck {
    function testExerciseReplayAttack() public {
        // Deploy the genuine SP1 Groth16 verifier.
        SP1Verifier verifier = new SP1Verifier();

        // Deploy the vulnerable starter contract with the real program vkey.
        ReplayableProofStarter c =
            new ReplayableProofStarter(address(verifier), ReplayableProofStarterFixture.PROGRAM_VKEY);

        // Submit the exact same proof/publicValues.
        c.submit(ReplayableProofStarterFixture.PROOF, ReplayableProofStarterFixture.PUBLIC_VALUES);

        // ============================== Begin Exercise ==============================================
        // TODO: Implement an attack here to increase your balance.


        // ============================== End Exercise ================================================

        // Assert the balance is double what it should be
        uint256 expectedBalance = uint256(c.FIXED_AMOUNT()) * 2;
        require(c.balances(c.FIXED_RECIPIENT()) == expectedBalance, "exercise failed");
    }
}
