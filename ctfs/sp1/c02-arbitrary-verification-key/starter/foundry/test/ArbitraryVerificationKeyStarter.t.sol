// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ArbitraryVerificationKeyStarter} from "../src/ArbitraryVerificationKeyStarter.sol";
import {ArbitraryVerificationKeyFixture} from "./fixtures/ArbitraryVerificationKeyFixture.sol";
import {FixtureBoundMockVerifier} from "./helpers/FixtureBoundMockVerifier.sol";

contract ArbitraryVerificationKeyStarterTest {
    function testVerifierAcceptsHonestAndAttackerPairs() public {
        FixtureBoundMockVerifier verifier = new FixtureBoundMockVerifier(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES),
            ArbitraryVerificationKeyFixture.ATTACKER_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES)
        );

        verifier.verifyProof(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES,
            ArbitraryVerificationKeyFixture.HONEST_PROOF
        );

        verifier.verifyProof(
            ArbitraryVerificationKeyFixture.ATTACKER_PROGRAM_VKEY,
            ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES,
            ArbitraryVerificationKeyFixture.ATTACKER_PROOF
        );

        (bool ok,) = address(verifier)
            .staticcall(
                abi.encodeWithSelector(
                    verifier.verifyProof.selector,
                    ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
                    ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES,
                    ArbitraryVerificationKeyFixture.ATTACKER_PROOF
                )
            );
        require(!ok, "attacker proof should fail under the honest vkey");
    }

    function testStarterAcceptsHonestClaim() public {
        FixtureBoundMockVerifier verifier = new FixtureBoundMockVerifier(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES),
            ArbitraryVerificationKeyFixture.ATTACKER_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES)
        );
        ArbitraryVerificationKeyStarter c = new ArbitraryVerificationKeyStarter(address(verifier));

        c.submit(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            ArbitraryVerificationKeyFixture.HONEST_PROOF,
            ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES
        );

        require(
            c.balances(ArbitraryVerificationKeyFixture.HONEST_RECIPIENT)
                == uint256(ArbitraryVerificationKeyFixture.HONEST_AMOUNT),
            "honest claim should credit the canonical recipient"
        );
    }
}
