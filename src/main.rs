use std::{ops::Deref, rc::Rc};

use log::info;
use symbolic_stack_machines::{
    calldata::Calldata,
    instructions::parse_bytecode_thread_local,
    machine::{
        assertions::ASSERTION_FAILURE, check_post_condition::check_post_condition_violated, Machine,
    },
    test_data::{
        PRIMALITY_CHECK_BYTECODE, PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR, SAFE_ADD_BYTECODE,
        SAFE_ADD_FUNCTION_SELECTOR_ARR,
    },
};

pub fn main() {
    env_logger::init();

    // primality_check_example();
    safe_add_example();
}

#[allow(dead_code)]
fn primality_check_example() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic_vars(
        PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR,
        vec![("x".into(), 4), ("y".into(), 36)],
    ));

    let calldata_s = Into::<String>::into(m.calldata.deref().clone());
    info!("symbolic_calldata: {}", calldata_s);

    let res = m.run_sym_check_assertions(Some(vec![ASSERTION_FAILURE]));

    let reverted = res.find_reverted(ASSERTION_FAILURE.into()).unwrap();

    let byte_solutions = &reverted.solve_results.as_ref().unwrap().bytes;

    let concrete_calldata = reverted.calldata.solve(byte_solutions);

    info!("symbolic_calldata: {}", calldata_s);
    info!("concrete_calldata: {}", concrete_calldata);
}

#[allow(dead_code)]
fn safe_add_example() {
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

    info!("post condition violated: {}", post_condition_violated);
}
