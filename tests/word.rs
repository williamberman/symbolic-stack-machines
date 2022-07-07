use std::rc::Rc;

use symbolic_stack_machines::{instructions::{push1, calldataload, stop, shr, lit}, machine::Machine, calldata::Calldata, val::word::Word};

#[test]
pub fn test_shr_optimization() {
    let pgm = vec![
        push1(),
        lit(0),
        calldataload(),
        push1(),
        lit(224),
        shr(),
        stop()
    ];

    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Calldata::symbolic([0x01, 0x02, 0x03, 0x04], 64));

    let res = m.run_sym();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

    let done = res.leaves.get(0).unwrap();

    assert_eq!(done.stack.peek().unwrap(), &Word::C(16909060.into()));
}
