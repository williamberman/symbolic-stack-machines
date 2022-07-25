use std::rc::Rc;

use symbolic_stack_machines::{
    instructions::parse_bytecode_thread_local,
    machine::{mem_ptr::MemPtr, Machine},
    test_data::{RETURN_CONSTANT_BYTECODE, RETURN_CONSTANT_FUNCTION_SELECTOR_ARR},
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
