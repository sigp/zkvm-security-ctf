// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract FixtureBoundMockVerifier is ISP1Verifier {
    error InvalidMockProof();

    mapping(bytes32 => bytes32) public expectedPublicValuesHash;

    constructor(
        bytes32 honestProgramVKey,
        bytes32 honestPublicValuesHash,
        bytes32 attackerProgramVKey,
        bytes32 attackerPublicValuesHash
    ) {
        expectedPublicValuesHash[honestProgramVKey] = honestPublicValuesHash;
        expectedPublicValuesHash[attackerProgramVKey] = attackerPublicValuesHash;
    }

    function verifyProof(bytes32 programVKey, bytes calldata publicValues, bytes calldata proofBytes) external view {
        if (proofBytes.length != 0) revert InvalidMockProof();
        if (expectedPublicValuesHash[programVKey] != keccak256(publicValues)) revert InvalidMockProof();
    }
}
