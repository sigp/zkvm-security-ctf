// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ArbitraryVerificationKeySolution} from "../src/ArbitraryVerificationKeySolution.sol";
import {ArbitraryVerificationKeyFixture} from "./fixtures/ArbitraryVerificationKeyFixture.sol";
import {FixtureBoundMockVerifier} from "./helpers/FixtureBoundMockVerifier.sol";

contract ArbitraryVerificationKeySolutionTest {
    function testPinnedVKeyAcceptsHonestProofAndRejectsAttackerProof() public {
        FixtureBoundMockVerifier verifier = new FixtureBoundMockVerifier(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES),
            ArbitraryVerificationKeyFixture.ATTACKER_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES)
        );
        ArbitraryVerificationKeySolution c =
            new ArbitraryVerificationKeySolution(address(verifier), ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY);

        c.submit(
            ArbitraryVerificationKeyFixture.HONEST_PROOF,
            ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES
        );

        require(
            c.balances(ArbitraryVerificationKeyFixture.HONEST_RECIPIENT)
                == uint256(ArbitraryVerificationKeyFixture.HONEST_AMOUNT),
            "honest proof should succeed"
        );

        (bool ok,) = address(c)
            .call(
                abi.encodeWithSelector(
                    c.submit.selector,
                    ArbitraryVerificationKeyFixture.ATTACKER_PROOF,
                    ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES
                )
            );
        require(!ok, "attacker proof should fail under the pinned honest vkey");
        require(c.balances(ArbitraryVerificationKeyFixture.ATTACKER_RECIPIENT) == 0, "attacker should not be credited");
    }
}
