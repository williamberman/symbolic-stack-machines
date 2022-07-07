use std::rc::Rc;

use log::info;
use symbolic_stack_machines::{
    calldata::Calldata,
    instructions::parse_bytecode,
    machine::Machine,
    misc::{
        PRIMALITY_CHECK_ASSERT_REVERT_STRING, PRIMALITY_CHECK_BYTECODE,
        PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR,
    },
};

pub fn main() {
    env_logger::init();

    let pgm = parse_bytecode(PRIMALITY_CHECK_BYTECODE);
    let mut m = Machine::new(pgm);

    let symbolic_calldata = Calldata::symbolic(PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR, 64);

    let symbolic_calldata_string = Into::<String>::into(symbolic_calldata.clone());

    info!("symbolic_calldata: {}", symbolic_calldata_string);

    m.calldata = Rc::new(symbolic_calldata);

    let res = m.run_sym();

    let reverted = res
        .find_reverted(PRIMALITY_CHECK_ASSERT_REVERT_STRING.into())
        .unwrap();

    let byte_solutions = &reverted.solve_results.as_ref().unwrap().bytes;

    let concrete_calldata = reverted.calldata.solve(byte_solutions);

    info!("symbolic_calldata: {}", symbolic_calldata_string);
    info!("concrete_calldata: {}", concrete_calldata);
}
