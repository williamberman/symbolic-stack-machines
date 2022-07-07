use std::{ops::Not, rc::Rc};

use im::Vector;
use symbolic_stack_machines::{
    calldata::Calldata,
    instructions::parse_bytecode,
    machine::{mem_ptr::MemPtr, Machine},
    misc::{
        PRIMALITY_CHECK_ASSERT_REVERT_STRING, PRIMALITY_CHECK_BYTECODE,
        PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR, PRIMALITY_CHECK_FUNCTION_SELECTOR_INT,
    },
    val::word::Word,
};

#[test]
pub fn test_primality_check_empty_calldata() {
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
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
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(vec![0_u8, 0, 0, 0].into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 1);

    // Attempts to takes jump to function but has wrong calldata
    // so is unsat
    let pruned = res.pruned.get(0).unwrap();
    let expected_constraint = Word::C(PRIMALITY_CHECK_FUNCTION_SELECTOR_INT.into())
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
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR).into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 1);

    // Pruned path where function selector in calldata not correct
    let pruned = res.pruned.get(0).unwrap();
    let expected_constraint = Word::C(PRIMALITY_CHECK_FUNCTION_SELECTOR_INT.into())
        ._eq_word(PRIMALITY_CHECK_FUNCTION_SELECTOR_INT.into())
        ._eq(Word::zero());
    assert_eq!(pruned.constraints, Vector::from(vec![expected_constraint]));

    // Reverts because calldata is too short
    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_min() {
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend([0_u8; 64].into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);

    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_max() {
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut arg = [0_u8; 32];
    // 973013 == 0x0ED8D5
    arg[29] = 0x0E;
    arg[30] = 0xD8;
    arg[31] = 0xD5;

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend(arg.iter());
    calldata.extend(arg.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);

    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_pass() {
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut arg = [0_u8; 32];
    arg[31] = 0x02;

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend(arg.iter());
    calldata.extend(arg.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);

    let returned = res.leaves.get(0).unwrap();

    assert_eq!(
        returned.return_string().unwrap(),
        "0000000000000000000000000000000000000000000000000000000000000539"
    );
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_fail() {
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
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

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend(arg1.into_iter());
    calldata.extend(arg2.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym();

    let reverted = res.leaves.get(0).unwrap();

    let revert = reverted.revert_ptr.clone().unwrap();

    assert_eq!(
        revert,
        MemPtr {
            offset: 0.into(),
            length: 36.into()
        }
    );

    assert_eq!(
        reverted.revert_string().unwrap(),
        PRIMALITY_CHECK_ASSERT_REVERT_STRING,
    );
}

#[test]
pub fn test_primality_check_arguments_symbolic() {
    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic(
        PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR,
        64,
    ));

    let res = m.run_sym();

    let reverted = res
        .find_reverted(PRIMALITY_CHECK_ASSERT_REVERT_STRING.into())
        .unwrap();

    let byte_solutions = &reverted.solve_results.as_ref().unwrap().bytes;
    
    let concrete_calldata = reverted.calldata.solve(byte_solutions);

    assert_eq!(concrete_calldata, "d5a2424900000000000000000000000000000000000000000000000000000000000003b900000000000000000000000000000000000000000000000000000000000003fd");
}
