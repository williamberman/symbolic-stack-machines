use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, push, sub, lit_32},
    machine::Machine,
    memory::Memory,
    stack::Stack, val::word::Word,
};

#[test]
fn test_simple() {
    let mut pgm = vec![];
    pgm.push(push());
    pgm.extend(lit_32(15));
    pgm.push(push());
    pgm.extend(lit_32(10));
    pgm.push(push());
    pgm.extend(lit_32(5));
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
        constraints: vec![],
    };

    let res = *machine.run().stack.peek().unwrap();

    assert_eq!(res, Word::from(0));
}
