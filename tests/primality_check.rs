use std::rc::Rc;

use symbolic_stack_machines::{
    calldata::Calldata,
    instructions::parse_bytecode_thread_local,
    machine::{assertions::ASSERTION_FAILURE, mem_ptr::MemPtr, Machine},
    test_data::{PRIMALITY_CHECK_BYTECODE, PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR},
};

#[test]
pub fn test_primality_check_empty_calldata() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let m = Machine::new(pgm);
    let res = m.run_sym_solve_at_each_branch();
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
pub fn test_primality_check_empty_calldata_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let m = Machine::new(pgm);
    let res = m.run_sym_inc();
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
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(vec![0_u8, 0, 0, 0].into());

    let res = m.run_sym_solve_at_each_branch();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

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
pub fn test_primality_check_wrong_calldata_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(vec![0_u8, 0, 0, 0].into());

    let res = m.run_sym_inc();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

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
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR).into());

    let res = m.run_sym_solve_at_each_branch();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

    // Reverts because calldata is too short
    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_zero_arguments_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR).into());

    let res = m.run_sym_inc();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

    // Reverts because calldata is too short
    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_min() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend([0_u8; 64].into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym_solve_at_each_branch();

    assert_eq!(res.leaves.len(), 1);

    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_min_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend([0_u8; 64].into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym_inc();

    assert_eq!(res.leaves.len(), 1);

    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_max() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
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

    let res = m.run_sym_solve_at_each_branch();

    assert_eq!(res.leaves.len(), 1);

    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_require_fail_max_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
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

    let res = m.run_sym_inc();

    assert_eq!(res.leaves.len(), 1);

    assert_eq!(res.leaves.get(0).unwrap().revert_string().unwrap(), "");
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_pass() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut arg = [0_u8; 32];
    arg[31] = 0x02;

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend(arg.iter());
    calldata.extend(arg.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym_solve_at_each_branch();

    assert_eq!(res.leaves.len(), 1);

    let returned = res.leaves.get(0).unwrap();

    assert_eq!(
        returned.return_string().unwrap(),
        "0000000000000000000000000000000000000000000000000000000000000539"
    );
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_pass_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let mut arg = [0_u8; 32];
    arg[31] = 0x02;

    let mut calldata = Vec::from(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR);
    calldata.extend(arg.iter());
    calldata.extend(arg.into_iter());

    m.calldata = Rc::new(calldata.into());

    let res = m.run_sym_inc();

    assert_eq!(res.leaves.len(), 1);

    let returned = res.leaves.get(0).unwrap();

    assert_eq!(
        returned.return_string().unwrap(),
        "0000000000000000000000000000000000000000000000000000000000000539"
    );
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_fail() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
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

    let res = m.run_sym_solve_at_each_branch();

    let reverted = res.leaves.get(0).unwrap();

    let revert = reverted.revert_ptr.clone().unwrap();

    assert_eq!(
        revert,
        MemPtr {
            offset: 0.into(),
            length: 36.into()
        }
    );

    assert_eq!(reverted.revert_string().unwrap(), ASSERTION_FAILURE,);
}

#[test]
pub fn test_primality_check_arguments_concrete_assert_fail_incremental_solver() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
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

    let res = m.run_sym_inc();

    let reverted = res.leaves.get(0).unwrap();

    let revert = reverted.revert_ptr.clone().unwrap();

    assert_eq!(
        revert,
        MemPtr {
            offset: 0.into(),
            length: 36.into()
        }
    );

    assert_eq!(reverted.revert_string().unwrap(), ASSERTION_FAILURE,);
}

#[test]
pub fn test_primality_check_arguments_symbolic() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic(
        PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR,
        64,
    ));

    // NOTE(will) - other sym run strats take too long to execute for tests
    let res = m.run_sym_check_assertions(Some(vec![ASSERTION_FAILURE]));

    let reverted = res.find_reverted(ASSERTION_FAILURE.into()).unwrap();

    let byte_solutions = &reverted.solve_results.as_ref().unwrap().bytes;

    let concrete_calldata = reverted.calldata.solve(byte_solutions);

    assert_eq!(concrete_calldata, "d5a2424900000000000000000000000000000000000000000000000000000000000003b900000000000000000000000000000000000000000000000000000000000003fd");
}
