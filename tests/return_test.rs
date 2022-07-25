use std::rc::Rc;

use symbolic_stack_machines::{
    instructions::parse_bytecode_thread_local,
    machine::{mem_ptr::MemPtr, Machine},
    test_data::{RETURN_CONSTANT_BYTECODE, RETURN_CONSTANT_FUNCTION_SELECTOR_ARR, RETURN_SYMBOLIC_BYTECODE, RETURN_SYMBOLIC_FUNCTION_SELECTOR_ARR}, calldata::Calldata, val::byte::Byte,
};

#[test]
pub fn test_return_constant() {
    let pgm = parse_bytecode_thread_local(&RETURN_CONSTANT_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Vec::from(RETURN_CONSTANT_FUNCTION_SELECTOR_ARR).into());

    let res = m.run_sym_solve_at_each_branch();

    assert_eq!(res.pruned.len(), 0);
    assert_eq!(res.leaves.len(), 1);

    let returned = res.leaves.get(0).unwrap();

    assert_eq!(
        returned.return_ptr.as_ref().unwrap(),
        &MemPtr {
            offset: 128.into(),
            length: 32.into()
        }
    );

    assert_eq!(
        returned.return_string().unwrap(),
        "0000000000000000000000000000000000000000000000000000000000000539"
    );
}

#[test]
pub fn test_return_symbolic() {
    let pgm = parse_bytecode_thread_local(&RETURN_SYMBOLIC_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic_vars(
        RETURN_SYMBOLIC_FUNCTION_SELECTOR_ARR,
        vec![("x".into(), 4)],
    ));

    let res = m.run_sym_returns();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 1);

    let returned = res.leaves.get(0).unwrap();

    let solve_results = returned.solve_results.as_ref().unwrap();

    let concrete_calldata = returned.calldata.solve(&solve_results.bytes);

    let concrete_return_value = Byte::solve(returned.return_bytes().unwrap(), &solve_results.bytes);

    assert_eq!(concrete_calldata, "b3de648b0000000000000000000000000000000000000000000000000000000000000001");
    assert_eq!(concrete_return_value, "0000000000000000000000000000000000000000000000000000000000000001");
}
