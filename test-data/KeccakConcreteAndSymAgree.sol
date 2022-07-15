// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract KeccakConcreteAndSymAgree {
    function kecc(uint256 x) public pure {
        if (x == 0) {
            assert(keccak256(abi.encode(x)) == keccak256(abi.encode(0)));
        }
    }
}
