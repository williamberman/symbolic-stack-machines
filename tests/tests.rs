use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, push, sub},
    machine::Machine,
    memory::Memory,
    stack::Stack, val::word::Word,
};

#[test]
fn test_simple() {
    let pgm = vec![push(), push(), push(), push(), add(), add(), sub()];
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
