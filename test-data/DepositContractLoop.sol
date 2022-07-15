// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract Deposit {
    function deposit(uint256 deposit_count) external pure {
        require(deposit_count < 2**32 - 1);
        ++deposit_count;
        bool found = false;
        for (uint256 height = 0; height < 32; height++) {
            if ((deposit_count & 1) == 1) {
                found = true;
                break;
            }
            deposit_count = deposit_count >> 1;
        }
        assert(found);
    }
}
