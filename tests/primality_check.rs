use std::{rc::Rc, ops::Not};

use im::Vector;
use symbolic_stack_machines::{instructions::parse_bytecode, machine::Machine, val::word::Word};

// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.9;
//
// contract PrimalityCheck {
//     function factor(uint x, uint y) public pure  {
//       require(1 < x && x < 973013 && 1 < y && y < 973013);
//       assert(x*y != 973013);
//     }
// }

// solc --bin-runtime -o . --overwrite PrimalityCheck.sol

// cat PrimalityCheck.bin-runtime
static BYTECODE: &str = "608060405234801561001057600080fd5b506004361061002b5760003560e01c8063d5a2424914610030575b600080fd5b61004a600480360381019061004591906100df565b61004c565b005b81600110801561005e5750620ed8d582105b801561006a5750806001105b80156100785750620ed8d581105b61008157600080fd5b620ed8d58183610091919061014e565b14156100a05761009f6101a8565b5b5050565b600080fd5b6000819050919050565b6100bc816100a9565b81146100c757600080fd5b50565b6000813590506100d9816100b3565b92915050565b600080604083850312156100f6576100f56100a4565b5b6000610104858286016100ca565b9250506020610115858286016100ca565b9150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000610159826100a9565b9150610164836100a9565b9250817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048311821515161561019d5761019c61011f565b5b828202905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052600160045260246000fdfea2646970667358221220c71687d33a6eefb62e6dac7c5790e19c31a4c69535db9b10eab27cb87eb434cd64736f6c634300080c0033";

static FUNCTION_SELECTOR: u32 = 3584180809;

#[test]
pub fn test_primality_check_empty_calldata() {
    let pgm = parse_bytecode(BYTECODE);
    let m = Machine::new(pgm);
    let res = m.run_sym();
    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

    let m = res.leaves.get(0).unwrap();

    assert_eq!(m.env.revert_offset, Some(0.into()));
    assert_eq!(m.env.revert_length, Some(0.into()));
}

#[test]
pub fn test_primality_check_wrong_calldata() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(vec![0_u8, 0, 0, 0].into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 1);

    // Attempts to takes jump to function but has wrong calldata
    // so is unsat
    let pruned = res.pruned.get(0).unwrap();
    let expected_constraint = Word::C(FUNCTION_SELECTOR.into())._eq_word(Word::zero())._eq(Word::zero()).not();
    assert_eq!(pruned.constraints, Vector::from(vec![expected_constraint]));

    // Reverts because wrong calldata
    let reverted = res.leaves.get(0).unwrap();
    assert_eq!(reverted.env.revert_offset, Some(0.into()));
    assert_eq!(reverted.env.revert_length, Some(0.into()));
}
