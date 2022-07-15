// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract Caller {
    uint256 x;
    Callee constant callee = Callee(0x35D1b3F3D7966A1DFe207aa4514C12a259A0492B);

    function call_Callee() public view {
        // should fail since a.x() can be anything
        assert(callee.x() == x);
    }
}

contract Callee {
    uint256 public x;
}
