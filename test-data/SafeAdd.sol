// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract SafeAdd {
    function add(uint256 x, uint256 y) public pure returns (uint256 z) {
        require((z = x + y) >= x);
    }
}
