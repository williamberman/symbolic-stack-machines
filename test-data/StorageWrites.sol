// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract StorageWrites {
    uint256 x;

    function f(uint256 y) public {
        unchecked {
            x += y;
            x += y;
        }
    }
}
