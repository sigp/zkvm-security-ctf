// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

library IntegerOverflowFixture {
    address internal constant HONEST_RECIPIENT = 0x3333333333333333333333333333333333333333;
    uint64 internal constant HONEST_PRICE = 10;
    uint64 internal constant HONEST_QUANTITY = 3;
    uint64 internal constant HONEST_TOTAL = 30;
    bytes32 internal constant PROGRAM_VKEY = bytes32(0);
    bytes internal constant PUBLIC_VALUES = hex"";
    bytes internal constant PROOF = hex"";
}
