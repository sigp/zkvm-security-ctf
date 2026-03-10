// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {UnconstrainedVerifierStarter} from "../src/UnconstrainedVerifierStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {UnconstrainedVerifierFixture} from "./fixtures/UnconstrainedVerifierFixture.sol";

contract UnconstrainedVerifierStarterTest is Test {
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

    function testStarterRejectsExactReplayButAcceptsForeignChainClaim() public {
        vm.chainId(UnconstrainedVerifierFixture.HONEST_CHAIN_ID);

        SP1Verifier verifier = new SP1Verifier();
        UnconstrainedVerifierStarter c =
            new UnconstrainedVerifierStarter(address(verifier), UnconstrainedVerifierFixture.PROGRAM_VKEY);

        c.submit(UnconstrainedVerifierFixture.HONEST_PROOF, UnconstrainedVerifierFixture.HONEST_PUBLIC_VALUES);

        vm.expectRevert(UnconstrainedVerifierStarter.Replay.selector);
        c.submit(UnconstrainedVerifierFixture.HONEST_PROOF, UnconstrainedVerifierFixture.HONEST_PUBLIC_VALUES);

        uint64 foreignChainId = c.claimChainId(UnconstrainedVerifierFixture.FOREIGN_PUBLIC_VALUES);
        assertEq(uint256(foreignChainId), uint256(UnconstrainedVerifierFixture.FOREIGN_CHAIN_ID));
        assertTrue(uint256(foreignChainId) != block.chainid);

        c.submit(UnconstrainedVerifierFixture.FOREIGN_PROOF, UnconstrainedVerifierFixture.FOREIGN_PUBLIC_VALUES);

        assertEq(
            uint256(c.balances(UnconstrainedVerifierFixture.FIXED_RECIPIENT)),
            uint256(UnconstrainedVerifierFixture.FIXED_AMOUNT) * 2
        );
    }
}
