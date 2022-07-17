// hevm parity tests

use std::rc::Rc;

use symbolic_stack_machines::{test_data::{SAFE_ADD_BYTECODE, SAFE_ADD_FUNCTION_SELECTOR_ARR}, instructions::parse_bytecode_thread_local, machine::Machine, calldata::Calldata, val::word::Word};

// #[test]
pub fn safe_add() {
    let pgm = parse_bytecode_thread_local(&SAFE_ADD_BYTECODE);

    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic(
        SAFE_ADD_FUNCTION_SELECTOR_ARR,
        64,
    ));

    let x = m.calldata.read_word(4.into());
    let y = m.calldata.read_word(36.into());

    // m.constraints.push_back(x.clone()._lt_eq(x.clone() + y.clone())._eq(Word::one()));
    m.constraints.push_back(x.clone()._eq(y.clone()));

    let res = m.run_sym(None);

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());

    let returned = res.leaves.get(0).unwrap();

    dbg!(returned.return_string());
    dbg!(returned.revert_string());
}
