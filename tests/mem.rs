use im::Vector;
use symbolic_stack_machines::{
    instructions::{lit, mstore, push1, stop},
    machine::Machine,
    val::byte::Byte,
};

#[test]
fn test_mstore() {
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
