use im::Vector;
use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, lit, lit_sym, push1, sub},
    machine::Machine,
    memory::Memory,
    stack::Stack,
    val::{word::Word, byte::Byte},
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

#[test]
fn test_symbolic() {
    let mut pgm = vec![];
    pgm.push(push1());
    pgm.push(lit_sym("x".into()));
    pgm.push(push1());
    pgm.push(lit(2));
    pgm.push(push1());
    pgm.push(lit(3));
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

    let mut sym = [0; 32].map(|_| Byte::C(0));
    sym[31] = Byte::S("x".into());

    let expected = Word::Sub(Box::new(Word::from(5)), Box::new(Word::Concat(sym)));

    assert_eq!(res, expected);
}
