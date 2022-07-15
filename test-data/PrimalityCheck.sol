// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract PrimalityCheck {
    function factor(uint x, uint y) public pure returns(uint) {
      require(1 < x && x < 973013 && 1 < y && y < 973013);
      assert(x*y != 973013);
      return 1337;
    }
}
