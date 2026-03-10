// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

import {IntegerOverflowStarter} from "../src/IntegerOverflowStarter.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {IntegerOverflowFixture} from "./fixtures/IntegerOverflowFixture.sol";

contract IntegerOverflowStarterTest is Test {
    function testVerifierAcceptsHonestProof() public {
        SP1Verifier verifier = new SP1Verifier();

        verifier.verifyProof(
            IntegerOverflowFixture.PROGRAM_VKEY,
            IntegerOverflowFixture.HONEST_PUBLIC_VALUES,
            IntegerOverflowFixture.HONEST_PROOF
        );
    }

    function testHonestSubmitMintsExpectedQuantity() public {
        SP1Verifier verifier = new SP1Verifier();
        IntegerOverflowStarter c =
            new IntegerOverflowStarter(address(verifier), IntegerOverflowFixture.PROGRAM_VKEY);

        c.submit{value: IntegerOverflowFixture.HONEST_TOTAL}(
            IntegerOverflowFixture.HONEST_PROOF, IntegerOverflowFixture.HONEST_PUBLIC_VALUES
        );

        assertEq(
            c.tokenBalances(IntegerOverflowFixture.HONEST_RECIPIENT),
            uint256(IntegerOverflowFixture.HONEST_QUANTITY)
        );
    }

    function testRejectsExactReplay() public {
        SP1Verifier verifier = new SP1Verifier();
        IntegerOverflowStarter c =
            new IntegerOverflowStarter(address(verifier), IntegerOverflowFixture.PROGRAM_VKEY);

        c.submit{value: IntegerOverflowFixture.HONEST_TOTAL}(
            IntegerOverflowFixture.HONEST_PROOF, IntegerOverflowFixture.HONEST_PUBLIC_VALUES
        );

        vm.expectRevert(IntegerOverflowStarter.Replay.selector);
        c.submit{value: IntegerOverflowFixture.HONEST_TOTAL}(
            IntegerOverflowFixture.HONEST_PROOF, IntegerOverflowFixture.HONEST_PUBLIC_VALUES
        );
    }

    function testRejectsInvalidPublicValuesLength() public {
        SP1Verifier verifier = new SP1Verifier();
        IntegerOverflowStarter c =
            new IntegerOverflowStarter(address(verifier), IntegerOverflowFixture.PROGRAM_VKEY);

        vm.expectRevert(IntegerOverflowStarter.InvalidPublicValues.selector);
        c.submit(hex"1234", hex"1234");
    }
}
