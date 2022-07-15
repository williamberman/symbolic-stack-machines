// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract CalldataBeyondCalldataSize {
    function f() public pure {
        uint256 y;
        assembly {
            let x := calldatasize()
            y := calldataload(x)
        }
        assert(y == 0);
    }
}
