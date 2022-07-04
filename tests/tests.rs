use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, push, sub},
    machine::Machine,
    memory::Memory,
    stack::{Stack, StackVal},
};

#[test]
fn test_simple() {
    let pgm = vec![push(15), push(5), push(5), push(5), add(), add(), sub()];
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

    let res = *machine.run().stack.peek(0).unwrap();

    assert_eq!(res, StackVal::from(0));
}
