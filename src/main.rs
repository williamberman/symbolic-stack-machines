use std::rc::Rc;

use log::info;
use symbolic_stack_machines::{
    calldata::Calldata,
    instructions::parse_bytecode_thread_local,
    machine::{assertions::ASSERTION_FAILURE, Machine},
    test_data::{
        PRIMALITY_CHECK_BYTECODE, PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR, SAFE_ADD_BYTECODE,
        SAFE_ADD_FUNCTION_SELECTOR_ARR,
    },
    val::word::Word,
};

pub fn main() {
    env_logger::init();

    safe_add_example();
}

#[allow(dead_code)]
fn primality_check_example() {
    let pgm = parse_bytecode_thread_local(&PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let symbolic_calldata = Calldata::symbolic(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR, 64);

    let symbolic_calldata_string = Into::<String>::into(symbolic_calldata.clone());

    info!("symbolic_calldata: {}", symbolic_calldata_string);

    m.calldata = Rc::new(symbolic_calldata);

    let res = m.run_sym(Some(vec![ASSERTION_FAILURE]));

    let reverted = res.find_reverted(ASSERTION_FAILURE.into()).unwrap();

    let byte_solutions = &reverted.solve_results.as_ref().unwrap().bytes;

    let concrete_calldata = reverted.calldata.solve(byte_solutions);

    info!("symbolic_calldata: {}", symbolic_calldata_string);
    info!("concrete_calldata: {}", concrete_calldata);
}

#[allow(dead_code)]
fn safe_add_example() {
    let pgm = parse_bytecode_thread_local(&SAFE_ADD_BYTECODE);

    let mut m = Machine::new(pgm);

    let mut calldata = Calldata::symbolic(SAFE_ADD_FUNCTION_SELECTOR_ARR, 64);

    calldata.vars = Some(vec![("x".into(), 4), ("y".into(), 36)]);

    m.calldata = Rc::new(calldata);

    let vars = m.calldata.variables_name_lookup();

    let x = vars.get("x").unwrap();
    let y = vars.get("y").unwrap();

    m.constraints
        .push_back(x.clone()._lt_eq(x.clone() + y.clone())._eq(Word::one()));
    m.constraints.push_back(x.clone()._eq(y.clone()));

    let res = m.run_sym(None);

    dbg!(res.leaves.len());
    dbg!(res.pruned.len());

    let returned = res.leaves.get(0).unwrap();

    dbg!(returned.return_string());
    dbg!(returned.revert_string());
}
