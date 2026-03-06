// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ArbitraryVerificationKeyStarter} from "../src/ArbitraryVerificationKeyStarter.sol";
import {ArbitraryVerificationKeyFixture} from "./fixtures/ArbitraryVerificationKeyFixture.sol";
import {FixtureBoundMockVerifier} from "./helpers/FixtureBoundMockVerifier.sol";

/// @notice Knowledge-check scaffold for C02.
/// @dev This exercise is expected to fail until the TODO is implemented.
contract ArbitraryVerificationKeyKnowledgeCheck {
    function testExerciseAttackerControlledVKey() public {
        // Deploy a verifier harness seeded with the honest and attacker fixture pairs.
        FixtureBoundMockVerifier verifier = new FixtureBoundMockVerifier(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES),
            ArbitraryVerificationKeyFixture.ATTACKER_PROGRAM_VKEY,
            keccak256(ArbitraryVerificationKeyFixture.ATTACKER_PUBLIC_VALUES)
        );

        // Deploy the vulnerable starter contract.
        ArbitraryVerificationKeyStarter c = new ArbitraryVerificationKeyStarter(address(verifier));

        // First submit the intended proof under the intended program vkey (we're just ensure the contract works).
        c.submit(
            ArbitraryVerificationKeyFixture.HONEST_PROGRAM_VKEY,
            ArbitraryVerificationKeyFixture.HONEST_PROOF,
            ArbitraryVerificationKeyFixture.HONEST_PUBLIC_VALUES
        );

        // ============================== Begin Exercise ==============================================
        // TODO: Implement the attack here.


        // ============================== End Exercise ================================================

        require(
            c.balances(ArbitraryVerificationKeyFixture.HONEST_RECIPIENT)
                == uint256(ArbitraryVerificationKeyFixture.HONEST_AMOUNT),
            "baseline honest submit failed"
        );
        require(
            c.balances(ArbitraryVerificationKeyFixture.ATTACKER_RECIPIENT)
                == uint256(ArbitraryVerificationKeyFixture.ATTACKER_AMOUNT),
            "exercise failed"
        );
    }
}
