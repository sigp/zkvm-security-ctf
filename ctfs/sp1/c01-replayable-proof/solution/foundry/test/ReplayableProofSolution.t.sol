// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ReplayableProofSolution} from "../src/ReplayableProofSolution.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {ReplayableProofSolutionFixture} from "./fixtures/ReplayableProofSolutionFixture.sol";

contract ReplayableProofSolutionTest {
    function testVerifierAcceptsValidAndRejectsInvalid() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            ReplayableProofSolutionFixture.PROGRAM_VKEY,
            ReplayableProofSolutionFixture.PUBLIC_VALUES,
            ReplayableProofSolutionFixture.PROOF
        );

        bytes memory badProof = abi.encodePacked(ReplayableProofSolutionFixture.PROOF);
        badProof[badProof.length - 1] = bytes1(uint8(badProof[badProof.length - 1]) ^ 0x01);

        (bool ok,) = address(verifier)
            .staticcall(
                abi.encodeWithSelector(
                    verifier.verifyProof.selector,
                    ReplayableProofSolutionFixture.PROGRAM_VKEY,
                    ReplayableProofSolutionFixture.PUBLIC_VALUES,
                    badProof
                )
            );
        require(!ok, "invalid proof should fail verification");
    }

    function testReplayIsRejectedInSolution() public {
        SP1Verifier verifier = new SP1Verifier();
        ReplayableProofSolution c =
            new ReplayableProofSolution(address(verifier), ReplayableProofSolutionFixture.PROGRAM_VKEY);

        c.submit(ReplayableProofSolutionFixture.PROOF, ReplayableProofSolutionFixture.PUBLIC_VALUES);

        (bool ok,) = address(c)
            .call(
                abi.encodeWithSelector(
                    c.submit.selector,
                    ReplayableProofSolutionFixture.PROOF,
                    ReplayableProofSolutionFixture.PUBLIC_VALUES
                )
            );
        require(!ok, "second submit should revert");

        uint64 expectedBalance = c.FIXED_AMOUNT();
        require(c.balances(c.FIXED_RECIPIENT()) == expectedBalance, "balance should be unchanged on replay");
    }
}
