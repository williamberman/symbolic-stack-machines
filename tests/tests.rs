use im::Vector;
use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, push1, sub, lit},
    machine::Machine,
    memory::Memory,
    stack::Stack, val::word::Word,
};

#[test]
fn test_simple() {
    let mut pgm = vec![];
    pgm.push(push1());
    pgm.push(lit(15));
    pgm.push(push1());
    pgm.push(lit(10));
    pgm.push(push1());
    pgm.push(lit(2));
    pgm.push(push1());
    pgm.push(lit(3));
    pgm.push(add());
    pgm.push(add());
    pgm.push(sub());

    let env = Env {};
    let pc = Some(0);
    let mem = Memory::default();
    let stack = Stack::default();
    let machine = Machine {
        stack,
        mem,
        env,
        pc,
        pgm,
        constraints: Vector::new(),
    };

    let res = machine.run().stack.peek().unwrap().clone();

    assert_eq!(res, Word::from(0));
}
