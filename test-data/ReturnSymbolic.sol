// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract ReturnSymbolic {
    function f(uint256 x) public pure returns (uint256) {
        assert(x == 1);
        return x;
    }
}
