// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {IntegerOverflowSolution} from "../src/IntegerOverflowSolution.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {IntegerOverflowFixture} from "./fixtures/IntegerOverflowFixture.sol";

contract IntegerOverflowSolutionTest is Test {
    function testVerifierAcceptsTheCanonicalProof() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            IntegerOverflowFixture.PROGRAM_VKEY,
            IntegerOverflowFixture.PUBLIC_VALUES,
            IntegerOverflowFixture.PROOF
        );
    }

    function testSolutionAcceptsHonestClaimAndRejectsReplay() public {
        SP1Verifier verifier = new SP1Verifier();
        IntegerOverflowSolution c =
            new IntegerOverflowSolution(address(verifier), IntegerOverflowFixture.PROGRAM_VKEY);

        c.submit{value: IntegerOverflowFixture.HONEST_TOTAL}(
            IntegerOverflowFixture.PROOF, IntegerOverflowFixture.PUBLIC_VALUES
        );

        assertEq(
            c.tokenBalances(IntegerOverflowFixture.HONEST_RECIPIENT),
            uint256(IntegerOverflowFixture.HONEST_QUANTITY)
        );

        vm.expectRevert(IntegerOverflowSolution.Replay.selector);
        c.submit{value: IntegerOverflowFixture.HONEST_TOTAL}(
            IntegerOverflowFixture.PROOF, IntegerOverflowFixture.PUBLIC_VALUES
        );
    }
}
