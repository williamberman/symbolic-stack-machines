// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract MultipleContractsCaller {
    uint256 x;
    MultipleContractsCallee constant callee = MultipleContractsCallee(0x35D1b3F3D7966A1DFe207aa4514C12a259A0492B);

    function call_Callee() public view {
        // should fail since a.x() can be anything
        assert(callee.x() == x);
    }
}

contract MultipleContractsCallee {
    uint256 public x;
}
