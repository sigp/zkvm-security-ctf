// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract ReplayableProofSolution {
    error InvalidProof();
    error Replay();

    address public constant FIXED_RECIPIENT = 0x3333333333333333333333333333333333333333;
    uint64 public constant FIXED_AMOUNT = 100;
    bytes32 internal constant NULLIFIER_DOMAIN_SEPARATOR = keccak256("C01:nullifier:v1");

    ISP1Verifier public immutable verifier;
    bytes32 public immutable programVKey;
    mapping(address => uint64) public balances;
    mapping(bytes32 => bool) public usedNullifiers;

    constructor(address _verifier, bytes32 _programVKey) {
        verifier = ISP1Verifier(_verifier);
        programVKey = _programVKey;
    }

    function computeNullifier(bytes calldata publicValues) public pure returns (bytes32) {
        return keccak256(abi.encode(NULLIFIER_DOMAIN_SEPARATOR, keccak256(publicValues)));
    }

    function submit(bytes calldata proof, bytes calldata publicValues) external {
        verifier.verifyProof(programVKey, publicValues, proof);

        bytes32 nullifier = computeNullifier(publicValues);
        if (usedNullifiers[nullifier]) revert Replay();
        usedNullifiers[nullifier] = true;

        balances[FIXED_RECIPIENT] += FIXED_AMOUNT;
    }
}
