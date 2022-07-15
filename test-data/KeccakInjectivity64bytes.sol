// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract KeccakInjectivity64Bytes {
    function f(
        uint256 x,
        uint256 y,
        uint256 w,
        uint256 z
    ) public pure {
        assert(
            keccak256(abi.encodePacked(x, y)) !=
                keccak256(abi.encodePacked(w, z))
        );
    }
}
