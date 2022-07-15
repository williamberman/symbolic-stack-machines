// hevm parity tests

use std::rc::Rc;

use symbolic_stack_machines::{test_data::{SAFE_ADD_BYTECODE, SAFE_ADD_FUNCTION_SELECTOR_ARR}, instructions::parse_bytecode_thread_local, machine::Machine, calldata::Calldata};

#[test]
pub fn safe_add() {
    let pgm = parse_bytecode_thread_local(&SAFE_ADD_BYTECODE);
    let mut m = Machine::new(pgm);
    m.calldata = Rc::new(Calldata::symbolic(
        SAFE_ADD_FUNCTION_SELECTOR_ARR,
        64,
    ));
    let res = m.run_sym(None);

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());
}
