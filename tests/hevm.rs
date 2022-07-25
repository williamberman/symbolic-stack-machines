// hevm parity tests

use std::rc::Rc;

use symbolic_stack_machines::{
    calldata::Calldata,
    instructions::parse_bytecode_thread_local,
    machine::{check_post_condition::check_post_condition_violated, Machine},
    test_data::{SAFE_ADD_BYTECODE, SAFE_ADD_FUNCTION_SELECTOR_ARR},
};

#[test]
pub fn safe_add_success() {
    let pgm = parse_bytecode_thread_local(&SAFE_ADD_BYTECODE);

    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic_vars(
        SAFE_ADD_FUNCTION_SELECTOR_ARR,
        vec![("x".into(), 4), ("y".into(), 36)],
    ));

    let vars = m.calldata.variables_name_lookup();

    let x = vars.get("x").unwrap().clone();
    let y = vars.get("y").unwrap().clone();

    m.constraints
        .push_back(x.clone()._lt_eq(x.clone() + y.clone()).into());

    let res = m.run_sym();

    let post_condition_violated = check_post_condition_violated(
        &res.leaves,
        |m| m.returned(),
        |m| vec![m.return_word().unwrap()._eq(x.clone() + y.clone())],
    );

    assert_eq!(post_condition_violated, false);
}

#[test]
pub fn safe_add() {
    let pgm = parse_bytecode_thread_local(&SAFE_ADD_BYTECODE);

    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic_vars(
        SAFE_ADD_FUNCTION_SELECTOR_ARR,
        vec![("x".into(), 4), ("y".into(), 36)],
    ));

    let vars = m.calldata.variables_name_lookup();

    let x = vars.get("x").unwrap();
    let y = vars.get("y").unwrap();

    m.constraints
        .push_back(x.clone()._lt_eq(x.clone() + y.clone()).into());
    m.constraints.push_back(x.clone()._eq(y.clone()));

    let res = m.run_sym();

    let post_condition_violated = check_post_condition_violated(
        &res.leaves,
        |m| m.returned(),
        |m| vec![m.return_word().unwrap()._eq(y.clone() * 2.into())],
    );

    assert_eq!(post_condition_violated, false);
}
