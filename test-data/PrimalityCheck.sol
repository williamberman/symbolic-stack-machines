// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract PrimalityCheck {
    function factor(uint256 x, uint256 y) public pure returns (uint256) {
        require(1 < x && x < 973013 && 1 < y && y < 973013);
        assert(x * y != 973013);
        return 1337;
    }
}
