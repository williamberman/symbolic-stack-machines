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

#[test]
fn test_im() {
    let mut v = im::Vector::new();

    v.push_back(1);
    v.push_back(2);
    v.push_back(3);

    dbg!("*********");
    dbg!(&v);

    let mut w = v.clone();

    dbg!("*********");
    dbg!(&v);
    dbg!(&w);

    w.pop_back();
    w.push_back(4);

    dbg!("*********");
    dbg!(&v);
    dbg!(&w);
}