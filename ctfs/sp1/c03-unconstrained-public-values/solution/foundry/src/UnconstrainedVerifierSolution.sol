// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract UnconstrainedVerifierSolution {
    error InvalidPublicValues();
    error InvalidChainId();
    error Replay();

    uint256 internal constant PUBLIC_VALUES_LENGTH = 80;
    bytes32 internal constant NULLIFIER_DOMAIN_SEPARATOR = keccak256("C03:nullifier:v1");

    address public constant FIXED_RECIPIENT = 0x3333333333333333333333333333333333333333;
    uint64 public constant FIXED_AMOUNT = 100;

    ISP1Verifier public immutable verifier;
    bytes32 public immutable programVKey;
    mapping(address => uint64) public balances;
    mapping(bytes32 => bool) public usedNullifiers;

    constructor(address _verifier, bytes32 _programVKey) {
        verifier = ISP1Verifier(_verifier);
        programVKey = _programVKey;
    }

    function claimChainId(bytes calldata publicValues) public pure returns (uint64) {
        if (publicValues.length != PUBLIC_VALUES_LENGTH) revert InvalidPublicValues();
        return _readUint64LE(publicValues, 0);
    }

    function computeNullifier(bytes calldata publicValues) public pure returns (bytes32) {
        return keccak256(abi.encode(NULLIFIER_DOMAIN_SEPARATOR, keccak256(publicValues)));
    }

    function submit(bytes calldata proof, bytes calldata publicValues) external {
        if (publicValues.length != PUBLIC_VALUES_LENGTH) revert InvalidPublicValues();

        verifier.verifyProof(programVKey, publicValues, proof);

        if (uint256(claimChainId(publicValues)) != block.chainid) revert InvalidChainId();

        bytes32 nullifier = computeNullifier(publicValues);
        if (usedNullifiers[nullifier]) revert Replay();
        usedNullifiers[nullifier] = true;

        balances[FIXED_RECIPIENT] += FIXED_AMOUNT;
    }

    function _readUint64LE(bytes calldata data, uint256 offset) internal pure returns (uint64 value) {
        uint256 accumulator;
        for (uint256 i = 0; i < 8; ++i) {
            accumulator |= uint256(uint8(data[offset + i])) << (8 * i);
        }
        value = uint64(accumulator);
    }
}
