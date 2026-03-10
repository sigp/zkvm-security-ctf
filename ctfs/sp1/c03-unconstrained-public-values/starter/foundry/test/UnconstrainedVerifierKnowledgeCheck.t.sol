// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {UnconstrainedVerifierStarter} from "../src/UnconstrainedVerifierStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {UnconstrainedVerifierFixture} from "./fixtures/UnconstrainedVerifierFixture.sol";

/// @notice Knowledge-check scaffold for C03.
/// @dev This file is intentionally not executed by default because the function name does not start
/// `test`. Rename the function when you are ready to run it with `forge test`.
contract UnconstrainedVerifierKnowledgeCheck is Test {
    function exerciseForeignChainProofIsAccepted() public {
        vm.chainId(UnconstrainedVerifierFixture.HONEST_CHAIN_ID);

        SP1Verifier verifier = new SP1Verifier();
        UnconstrainedVerifierStarter c =
            new UnconstrainedVerifierStarter(address(verifier), UnconstrainedVerifierFixture.PROGRAM_VKEY);

        c.submit(UnconstrainedVerifierFixture.HONEST_PROOF, UnconstrainedVerifierFixture.HONEST_PUBLIC_VALUES);

        // ============================== Begin Exercise ==============================================
        // TODO: Submit the foreign-chain proof while the test is still running on the honest chain.


        // ============================== End Exercise ================================================

        assertEq(
            uint256(c.balances(UnconstrainedVerifierFixture.FIXED_RECIPIENT)),
            uint256(UnconstrainedVerifierFixture.FIXED_AMOUNT) * 2,
            "exercise failed"
        );
    }
}
