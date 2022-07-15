// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract StorageCollisions {
    function f(uint256 x, uint256 y) public {
        assembly {
            let newx := sub(sload(x), 1)
            let newy := add(sload(y), 1)
            sstore(x, newx)
            sstore(y, newy)
        }
    }
}
