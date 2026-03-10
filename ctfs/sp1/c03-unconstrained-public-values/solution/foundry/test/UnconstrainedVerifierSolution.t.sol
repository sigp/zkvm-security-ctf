// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {UnconstrainedVerifierSolution} from "../src/UnconstrainedVerifierSolution.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {UnconstrainedVerifierFixture} from "./fixtures/UnconstrainedVerifierFixture.sol";

contract UnconstrainedVerifierSolutionTest is Test {
    function testVerifierAcceptsHonestAndForeignProofs() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            UnconstrainedVerifierFixture.PROGRAM_VKEY,
            UnconstrainedVerifierFixture.HONEST_PUBLIC_VALUES,
            UnconstrainedVerifierFixture.HONEST_PROOF
        );

        verifier.verifyProof(
            UnconstrainedVerifierFixture.PROGRAM_VKEY,
            UnconstrainedVerifierFixture.FOREIGN_PUBLIC_VALUES,
            UnconstrainedVerifierFixture.FOREIGN_PROOF
        );
    }

    function testSolutionRejectsForeignChainClaim() public {
        vm.chainId(UnconstrainedVerifierFixture.HONEST_CHAIN_ID);

        SP1Verifier verifier = new SP1Verifier();
        UnconstrainedVerifierSolution c =
            new UnconstrainedVerifierSolution(address(verifier), UnconstrainedVerifierFixture.PROGRAM_VKEY);

        c.submit(UnconstrainedVerifierFixture.HONEST_PROOF, UnconstrainedVerifierFixture.HONEST_PUBLIC_VALUES);

        vm.expectRevert(UnconstrainedVerifierSolution.InvalidChainId.selector);
        c.submit(UnconstrainedVerifierFixture.FOREIGN_PROOF, UnconstrainedVerifierFixture.FOREIGN_PUBLIC_VALUES);

        assertEq(
            uint256(c.balances(UnconstrainedVerifierFixture.FIXED_RECIPIENT)),
            uint256(UnconstrainedVerifierFixture.FIXED_AMOUNT)
        );
    }

    function testSolutionAcceptsMatchingForeignChainClaim() public {
        vm.chainId(UnconstrainedVerifierFixture.FOREIGN_CHAIN_ID);

        SP1Verifier verifier = new SP1Verifier();
        UnconstrainedVerifierSolution c =
            new UnconstrainedVerifierSolution(address(verifier), UnconstrainedVerifierFixture.PROGRAM_VKEY);

        c.submit(UnconstrainedVerifierFixture.FOREIGN_PROOF, UnconstrainedVerifierFixture.FOREIGN_PUBLIC_VALUES);

        assertEq(
            uint256(c.balances(UnconstrainedVerifierFixture.FIXED_RECIPIENT)),
            uint256(UnconstrainedVerifierFixture.FIXED_AMOUNT)
        );
    }
}
