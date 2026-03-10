// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {HostSideValidationSolution} from "../src/HostSideValidationSolution.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {HostSideValidationFixture} from "./fixtures/HostSideValidationFixture.sol";

contract HostSideValidationSolutionTest is Test {
    function testVerifierAcceptsTheCanonicalProof() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            HostSideValidationFixture.PROGRAM_VKEY,
            HostSideValidationFixture.PUBLIC_VALUES,
            HostSideValidationFixture.PROOF
        );
    }

    function testSolutionAcceptsHonestClaimAndRejectsReplay() public {
        SP1Verifier verifier = new SP1Verifier();
        HostSideValidationSolution c =
            new HostSideValidationSolution(address(verifier), HostSideValidationFixture.PROGRAM_VKEY);

        c.submit(HostSideValidationFixture.PROOF, HostSideValidationFixture.PUBLIC_VALUES);

        assertEq(
            c.balances(HostSideValidationFixture.FIXED_RECIPIENT),
            uint256(HostSideValidationFixture.FIXED_AMOUNT)
        );

        vm.expectRevert(HostSideValidationSolution.Replay.selector);
        c.submit(HostSideValidationFixture.PROOF, HostSideValidationFixture.PUBLIC_VALUES);
    }
}
