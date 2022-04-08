use symbolic_stack_machines::{
    concrete_int,
    instructions::{
        helpers::{ADD, ISZERO, JUMPI, MLOAD, MSTORE, PUSH, STOP, SUB},
        DynConcreteVMInstruction,
    },
    machine::{concrete::ConcreteIntMachine, run_machine, Program},
    memory::concrete_index::MemConcreteIntToConcreteInt,
    stack::ConcreteIntStack,
};

fn test_helper(
    pgm: Program<DynConcreteVMInstruction<ConcreteIntStack, MemConcreteIntToConcreteInt>>,
    expected: concrete_int::Wraps,
) {
    let stack = ConcreteIntStack::new();
    let mem = MemConcreteIntToConcreteInt::new();
    let machine = ConcreteIntMachine::new(stack, mem, &pgm, None);

    assert_eq!(run_machine(machine), Option::Some(expected.into()))
}

#[test]
fn test_basic() {
    test_helper(vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()], 4);
}

#[test]
fn test_basic_mem() {
    test_helper(vec![PUSH(1), PUSH(0), MSTORE(), PUSH(0), MLOAD()], 1);
}

#[test]
fn test_basic_jumpi() {
    test_helper(
        vec![
            PUSH(1),
            PUSH(2),
            PUSH(3),
            ADD(),
            SUB(),
            PUSH(4),
            SUB(),
            ISZERO(),
            PUSH(12),
            JUMPI(),
            PUSH(100),
            STOP(),
            PUSH(200),
        ],
        200,
    );
}

#[test]
fn test_basic_jumpi_2() {
    test_helper(
        vec![
            PUSH(1),
            PUSH(2),
            PUSH(3),
            ADD(),
            SUB(),
            PUSH(5),
            SUB(),
            ISZERO(),
            PUSH(12),
            JUMPI(),
            PUSH(100),
            STOP(),
            PUSH(200),
        ],
        100,
    );
}
