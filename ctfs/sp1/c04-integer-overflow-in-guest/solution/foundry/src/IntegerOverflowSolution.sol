// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract IntegerOverflowSolution {
    error InvalidPublicValues();
    error IncorrectPayment();
    error Replay();

    uint256 internal constant PUBLIC_VALUES_LENGTH = 88;
    uint256 internal constant RECIPIENT_OFFSET = 40;
    uint256 internal constant QUANTITY_OFFSET = 68;
    uint256 internal constant TOTAL_OFFSET = 76;
    bytes32 internal constant NULLIFIER_DOMAIN_SEPARATOR = keccak256("C04:nullifier:v1");

    ISP1Verifier public immutable verifier;
    bytes32 public immutable programVKey;
    mapping(address => uint256) public tokenBalances;
    mapping(bytes32 => bool) public usedNullifiers;

    constructor(address _verifier, bytes32 _programVKey) {
        verifier = ISP1Verifier(_verifier);
        programVKey = _programVKey;
    }

    function decodeClaim(bytes calldata publicValues) public pure returns (address recipient, uint64 quantity, uint64 total) {
        if (publicValues.length != PUBLIC_VALUES_LENGTH) revert InvalidPublicValues();

        uint160 rawRecipient;
        for (uint256 i = 0; i < 20; ++i) {
            rawRecipient = (rawRecipient << 8) | uint160(uint8(publicValues[RECIPIENT_OFFSET + i]));
        }

        recipient = address(rawRecipient);
        quantity = _readUint64LE(publicValues, QUANTITY_OFFSET);
        total = _readUint64LE(publicValues, TOTAL_OFFSET);
    }

    function computeNullifier(bytes calldata publicValues) public pure returns (bytes32) {
        if (publicValues.length != PUBLIC_VALUES_LENGTH) revert InvalidPublicValues();
        return keccak256(abi.encode(NULLIFIER_DOMAIN_SEPARATOR, keccak256(publicValues)));
    }

    function submit(bytes calldata proof, bytes calldata publicValues) external payable {
        if (publicValues.length != PUBLIC_VALUES_LENGTH) revert InvalidPublicValues();

        verifier.verifyProof(programVKey, publicValues, proof);

        bytes32 nullifier = computeNullifier(publicValues);
        if (usedNullifiers[nullifier]) revert Replay();
        usedNullifiers[nullifier] = true;

        (address recipient, uint64 quantity, uint64 total) = decodeClaim(publicValues);
        if (msg.value != uint256(total)) revert IncorrectPayment();

        tokenBalances[recipient] += uint256(quantity);
    }

    function _readUint64LE(bytes calldata data, uint256 offset) internal pure returns (uint64 value) {
        uint256 accumulator;
        for (uint256 i = 0; i < 8; ++i) {
            accumulator |= uint256(uint8(data[offset + i])) << (8 * i);
        }
        value = uint64(accumulator);
    }
}
