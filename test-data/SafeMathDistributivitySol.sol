// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

contract SafeMathDistributivitySol {
    function distributivity(
        uint256 x,
        uint256 y,
        uint256 z
    ) public pure {
        assert(mul(x, add(y, z)) == add(mul(x, y), mul(x, z)));
    }

    function add(uint256 x, uint256 y) internal pure returns (uint256 z) {
        unchecked {
            require((z = x + y) >= x, "ds-math-add-overflow");
        }
    }

    function mul(uint256 x, uint256 y) internal pure returns (uint256 z) {
        unchecked {
            require(y == 0 || (z = x * y) / y == x, "ds-math-mul-overflow");
        }
    }
}
