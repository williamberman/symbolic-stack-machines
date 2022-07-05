use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, push1, sub, lit_32, Instruction, lit},
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
    pgm.push(lit(5));
    pgm.push(add());
    pgm.push(add());
    pgm.push(sub());

    dbg!(Instruction::as_bytes(pgm.clone()));

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
        constraints: vec![],
    };

    let res = *machine.run().stack.peek().unwrap();

    assert_eq!(res, Word::from(0));
}
