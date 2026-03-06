// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract ReplayableProofStarter {
    error InvalidProof();

    address public constant FIXED_RECIPIENT = 0x3333333333333333333333333333333333333333;
    uint64 public constant FIXED_AMOUNT = 100;

    ISP1Verifier public immutable verifier;
    bytes32 public immutable programVKey;
    mapping(address => uint256) public balances;

    constructor(address _verifier, bytes32 _programVKey) {
        verifier = ISP1Verifier(_verifier);
        programVKey = _programVKey;
    }

    // @audit this function is vulnerable why?
    function submit(bytes calldata proof, bytes calldata publicValues) external {
        try verifier.verifyProof(programVKey, publicValues, proof) {}
        catch {
            revert InvalidProof();
        }

        balances[FIXED_RECIPIENT] += FIXED_AMOUNT;
    }
}
