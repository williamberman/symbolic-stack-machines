// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract KeccakInjectivity {
    function f(uint256 x, uint256 y) public pure {
        if (keccak256(abi.encodePacked(x)) == keccak256(abi.encodePacked(y)))
            assert(x == y);
    }
}
