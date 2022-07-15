// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract KeccakSoundness {
    mapping(uint256 => mapping(uint256 => uint256)) maps;

    function f(uint256 x, uint256 y) public view {
        assert(maps[y][0] == maps[x][0]);
    }
}
