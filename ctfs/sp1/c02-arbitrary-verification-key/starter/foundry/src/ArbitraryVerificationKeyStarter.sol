// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract ArbitraryVerificationKeyStarter {
    error InvalidProof();
    error InvalidPublicValues();

    uint256 internal constant PUBLIC_VALUES_LENGTH = 80;
    uint256 internal constant RECIPIENT_OFFSET = 40;
    uint256 internal constant AMOUNT_OFFSET = 60;

    ISP1Verifier public immutable verifier;
    mapping(address => uint256) public balances;
    mapping(bytes32 => bool) public consumedPublicValues;

    constructor(address _verifier) {
        verifier = ISP1Verifier(_verifier);
    }

    // Extract amount and recipient from public values.
    function decodeClaim(bytes calldata publicValues) public pure returns (address recipient, uint64 amount) {
        if (publicValues.length != PUBLIC_VALUES_LENGTH) revert InvalidPublicValues();

        uint160 rawRecipient;
        for (uint256 i = 0; i < 20; ++i) {
            rawRecipient = (rawRecipient << 8) | uint160(uint8(publicValues[RECIPIENT_OFFSET + i]));
        }

        recipient = address(rawRecipient);
        amount = _readUint64LE(publicValues, AMOUNT_OFFSET);
    }

    // @audit this function is vulnerable why?
    function submit(bytes32 programVKey, bytes calldata proof, bytes calldata publicValues) external {
        // Verify proof for public values against a VKey
        verifier.verifyProof(programVKey, publicValues, proof);
        
        // Prevent replay
        consumedPublicValues[keccak256(publicValues)] = true;

        // Extract recipient and amount from public values and update balance
        (address recipient, uint64 amount) = decodeClaim(publicValues);
        balances[recipient] += amount;
    }

    function _readUint64LE(bytes calldata data, uint256 offset) internal pure returns (uint64 value) {
        uint256 accumulator;
        for (uint256 i = 0; i < 8; ++i) {
            accumulator |= uint256(uint8(data[offset + i])) << (8 * i);
        }
        value = uint64(accumulator);
    }
}
