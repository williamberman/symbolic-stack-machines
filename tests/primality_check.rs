use std::{ops::Not, rc::Rc};

use im::Vector;
use symbolic_stack_machines::{
    instructions::parse_bytecode,
    machine::{mem_ptr::MemPtr, Machine},
    val::word::Word,
};

// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.9;
//
// contract PrimalityCheck {
//     function factor(uint x, uint y) public pure returns(uint) {
//       require(1 < x && x < 973013 && 1 < y && y < 973013);
//       assert(x*y != 973013);
//       return 1337;
//     }
// }

// solc --bin-runtime -o . --overwrite PrimalityCheck.sol

// cat PrimalityCheck.bin-runtime
static BYTECODE: &str = "608060405234801561001057600080fd5b506004361061002b5760003560e01c8063d5a2424914610030575b600080fd5b61004a600480360381019061004591906100fc565b610060565b604051610057919061014b565b60405180910390f35b60008260011080156100745750620ed8d583105b80156100805750816001105b801561008e5750620ed8d582105b61009757600080fd5b620ed8d582846100a79190610195565b14156100b6576100b56101ef565b5b610539905092915050565b600080fd5b6000819050919050565b6100d9816100c6565b81146100e457600080fd5b50565b6000813590506100f6816100d0565b92915050565b60008060408385031215610113576101126100c1565b5b6000610121858286016100e7565b9250506020610132858286016100e7565b9150509250929050565b610145816100c6565b82525050565b6000602082019050610160600083018461013c565b92915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006101a0826100c6565b91506101ab836100c6565b9250817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156101e4576101e3610166565b5b828202905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052600160045260246000fdfea2646970667358221220b6b1339bfb75ad64c1352af183f05265b8d840c0225fa355a7848684a4cfbc4b64736f6c634300080c0033";

static FUNCTION_SELECTOR_INT: u32 = 3584180809;

static FUNCTION_SELECTOR_ARR: [u8; 4] = [0xd5_u8, 0xa2, 0x42, 0x49];

#[test]
pub fn test_primality_check_empty_calldata() {
    let pgm = parse_bytecode(BYTECODE);
    let m = Machine::new(pgm);
    let res = m.run_sym();
    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

    let reverted = res.leaves.get(0).unwrap();

    assert_eq!(
        reverted.revert_ptr,
        Some(MemPtr {
            offset: 0.into(),
            length: 0.into()
        })
    );
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
    let expected_constraint = Word::C(FUNCTION_SELECTOR_INT.into())
        ._eq_word(Word::zero())
        ._eq(Word::zero())
        .not();
    assert_eq!(pruned.constraints, Vector::from(vec![expected_constraint]));

    // Reverts because wrong calldata
    let reverted = res.leaves.get(0).unwrap();
    assert_eq!(
        reverted.revert_ptr,
        Some(MemPtr {
            offset: 0.into(),
            length: 0.into()
        })
    );
}

#[test]
pub fn test_primality_check_zero_arguments() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Vec::from(FUNCTION_SELECTOR_ARR).into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 1);

    // Pruned path where function selector in calldata not correct
    let pruned = res.pruned.get(0).unwrap();
    let expected_constraint = Word::C(FUNCTION_SELECTOR_INT.into())
        ._eq_word(FUNCTION_SELECTOR_INT.into())
        ._eq(Word::zero());
    assert_eq!(pruned.constraints, Vector::from(vec![expected_constraint]));

    // Reverts because calldata is too short
    let reverted = res.leaves.get(0).unwrap();
    assert_eq!(
        reverted.revert_ptr,
        Some(MemPtr {
            offset: 0.into(),
            length: 0.into()
        })
    );
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_min() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    let mut calldata = Vec::from(FUNCTION_SELECTOR_ARR);
    calldata.extend([0_u8; 64].into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());

    dbg!(&res.pruned.get(0).unwrap().constraints);
    dbg!(&res.pruned.get(1).unwrap().constraints);
    dbg!(&res.pruned.get(2).unwrap().constraints);

    todo!();
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_max() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    let mut arg = [0_u8; 32];
    // 973013 == 0x0ED8D5
    arg[29] = 0x0E;
    arg[30] = 0xD8;
    arg[31] = 0xD5;

    let mut calldata = Vec::from(FUNCTION_SELECTOR_ARR);
    calldata.extend(arg.iter());
    calldata.extend(arg.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());

    dbg!(&res.pruned.get(0).unwrap().constraints);
    dbg!(&res.pruned.get(1).unwrap().constraints);
    dbg!(&res.pruned.get(2).unwrap().constraints);

    todo!()
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_pass() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    let mut arg = [0_u8; 32];
    arg[31] = 0x02;

    let mut calldata = Vec::from(FUNCTION_SELECTOR_ARR);
    calldata.extend(arg.iter());
    calldata.extend(arg.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());

    dbg!(&res.leaves.get(0).unwrap().revert_ptr);

    todo!();
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_fail() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    // 953 * 1021 == 973013

    // 953 == 0x03B9
    let mut arg1 = [0_u8; 32];
    arg1[30] = 0x03;
    arg1[31] = 0xB9;

    // 1021 == 0x03FD
    let mut arg2 = [0_u8; 32];
    arg2[30] = 0x03;
    arg2[31] = 0xFD;

    let mut calldata = Vec::from(FUNCTION_SELECTOR_ARR);
    calldata.extend(arg1.into_iter());
    calldata.extend(arg2.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());

    let reverted = res.leaves.get(0).unwrap();

    let revert = reverted.revert_ptr.clone().unwrap();

    assert_eq!(
        revert,
        MemPtr {
            offset: 0.into(),
            length: 36.into()
        }
    );

    let revert_string = reverted.revert_string().unwrap();

    dbg!(revert_string);

    todo!()
}

#[test]
pub fn test_primality_check_arguments_symbolic() {
    todo!()
}
