// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ReplayableProofStarter} from "../src/ReplayableProofStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {ReplayableProofStarterFixture} from "./fixtures/ReplayableProofStarterFixture.sol";

contract ReplayableProofStarterTest {
    function testVerifierAcceptsValidAndRejectsInvalid() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            ReplayableProofStarterFixture.PROGRAM_VKEY,
            ReplayableProofStarterFixture.PUBLIC_VALUES,
            ReplayableProofStarterFixture.PROOF
        );

        bytes memory badProof = abi.encodePacked(ReplayableProofStarterFixture.PROOF);
        badProof[badProof.length - 1] = bytes1(uint8(badProof[badProof.length - 1]) ^ 0x01);

        (bool ok,) = address(verifier)
            .staticcall(
                abi.encodeWithSelector(
                    verifier.verifyProof.selector,
                    ReplayableProofStarterFixture.PROGRAM_VKEY,
                    ReplayableProofStarterFixture.PUBLIC_VALUES,
                    badProof
                )
            );
        require(!ok, "invalid proof should fail verification");
    }
}
