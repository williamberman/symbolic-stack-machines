// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract Caller {
    uint256 x;
    Callee callee;

    function call_Callee() public {
        callee = new Callee();
        // should fail since a.x() can be anything
        assert(callee.x() == x);
    }
}

contract Callee {
    uint256 public x;
}
