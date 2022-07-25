// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract MultipleContracts2Caller {
    uint256 x;
    MultipleContracts2Callee callee;

    function call_callee() public {
        callee = new MultipleContracts2Callee();
        // should fail since a.x() can be anything
        assert(callee.x() == x);
    }
}

contract MultipleContracts2Callee {
    uint256 public x;
}
