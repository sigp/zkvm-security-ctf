// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {HostSideValidationStarter} from "../src/HostSideValidationStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {HostSideValidationFixture} from "./fixtures/HostSideValidationFixture.sol";

contract HostSideValidationStarterTest is Test {
    function testVerifierAcceptsHonestProof() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            HostSideValidationFixture.PROGRAM_VKEY,
            HostSideValidationFixture.HONEST_PUBLIC_VALUES,
            HostSideValidationFixture.HONEST_PROOF
        );
    }

    function testHonestSubmitCreditsRecipient() public {
        SP1Verifier verifier = new SP1Verifier();
        HostSideValidationStarter c =
            new HostSideValidationStarter(address(verifier), HostSideValidationFixture.PROGRAM_VKEY);

        c.submit(HostSideValidationFixture.HONEST_PROOF, HostSideValidationFixture.HONEST_PUBLIC_VALUES);

        assertEq(
            c.balances(HostSideValidationFixture.HONEST_RECIPIENT),
            uint256(HostSideValidationFixture.HONEST_AMOUNT)
        );
    }

    function testRejectsExactReplay() public {
        SP1Verifier verifier = new SP1Verifier();
        HostSideValidationStarter c =
            new HostSideValidationStarter(address(verifier), HostSideValidationFixture.PROGRAM_VKEY);

        c.submit(HostSideValidationFixture.HONEST_PROOF, HostSideValidationFixture.HONEST_PUBLIC_VALUES);

        vm.expectRevert(HostSideValidationStarter.Replay.selector);
        c.submit(HostSideValidationFixture.HONEST_PROOF, HostSideValidationFixture.HONEST_PUBLIC_VALUES);
    }

    function testRejectsInvalidPublicValuesLength() public {
        SP1Verifier verifier = new SP1Verifier();
        HostSideValidationStarter c =
            new HostSideValidationStarter(address(verifier), HostSideValidationFixture.PROGRAM_VKEY);

        vm.expectRevert(HostSideValidationStarter.InvalidPublicValues.selector);
        c.submit(HostSideValidationFixture.HONEST_PROOF, hex"1234"); // Too short
    }
}
