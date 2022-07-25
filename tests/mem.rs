use im::Vector;
use primitive_types::U256;
use symbolic_stack_machines::{
    instructions::{lit, mstore, push1, stop},
    machine::Machine,
    val::byte::Byte,
};

#[test]
fn test_mstore_concrete() {
    let pgm = vec![push1(), lit(128), push1(), lit(64), mstore(), stop()];

    let machine = Machine::new(pgm);

    let res = machine.run();

    let actual = res.mem.inner();

    let expected_len = 64 + 32;

    assert_eq!(actual.len(), expected_len);

    let mut expected = Vector::from(vec![0_u8; expected_len]);
    expected[expected_len - 1] = 128;

    let expected: Vector<Byte> = expected.into_iter().map(|x| x.into()).collect();

    assert_eq!(actual, &expected);
}

#[test]
fn test_mstore_symbolic() {
    let pgm = vec![push1(), lit("x"), push1(), lit(0), mstore(), stop()];

    let machine = Machine::new(pgm);

    let res = machine.run_sym();

    assert_eq!(res.leaves.len(), 1);
    assert_eq!(res.pruned.len(), 0);

    let done = res.leaves.get(0).unwrap();

    let actual = done.mem.inner();

    let expected = Vector::from(vec![0_u8; 32]);
    let mut expected: Vector<Byte> = expected.into_iter().map(|x| x.into()).collect();
    expected[31] = "x".into();

    assert_eq!(actual, &expected);
}
