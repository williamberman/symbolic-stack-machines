use std::collections::HashMap;

use im::Vector;
use symbolic_stack_machines::{
    environment::Env,
    instructions::{add, iszero, jumpi, lit, push1, stop, sub, assert_ins, jump},
    machine::Machine,
    memory::Memory,
    stack::Stack,
    val::{byte::Byte, word::Word},
    z3::{solve_z3, SolveResults},
};

#[test]
fn test_simple() {
    let pgm = vec![
        push1(),
        lit(15),
        push1(),
        lit(10),
        push1(),
        lit(2),
        push1(),
        lit(3),
        add(),
        add(),
        sub(),
    ];

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
fn test_symbolic_single_machine() {
    let pgm = vec![
        push1(),
        lit("x"),
        push1(),
        lit(2),
        push1(),
        lit(3),
        add(),
        sub(),
    ];

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

#[test]
fn test_symbolic_multiple_machines() {
    let pgm = vec![
        push1(),
        lit(1),
        push1(),
        lit(2),
        push1(),
        lit("x"),
        add(),
        sub(),
        push1(),
        lit(4),
        sub(),
        iszero(),
        push1(),
        lit(18),
        jumpi(),
        push1(),
        lit(100),
        stop(),
        push1(),
        lit(200),
    ];

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

    let sym_results = machine.run_sym();

    assert_eq!(sym_results.pruned, vec![]);

    let ms = sym_results.leaves;

    assert_eq!(ms.len(), 2);

    let take_jump = ms.get(0).unwrap();
    let no_take_jump = ms.get(1).unwrap();

    assert_eq!(take_jump.stack.peek().unwrap(), &Word::from(200));

    assert_eq!(no_take_jump.stack.peek().unwrap(), &Word::from(100));

    let take_jump_sol = solve_z3(&take_jump.constraints, vec![], vec!["x".into()]).unwrap();

    assert_eq!(
        take_jump_sol,
        SolveResults {
            words: HashMap::new(),
            bytes: HashMap::from([("x".into(), 3)])
        }
    );

    let no_take_jump_sol = solve_z3(&no_take_jump.constraints, vec![], vec!["x".into()]).unwrap();

    assert_eq!(
        no_take_jump_sol,
        SolveResults {
            words: HashMap::new(),
            bytes: HashMap::from([("x".into(), 252)])
        }
    );
}

#[test]
fn test_symbolic_multiple_machines_filtered() {
    let pgm = vec![
        push1(),         // 0
        lit(1),          // 1
        push1(),         // 2
        lit(2),          // 3
        push1(),         // 4
        lit("x"),        // 5
        add(),           // 6
        sub(),           // 7
        push1(),         // 8
        lit(4),          // 9
        sub(),           // 10
        iszero(),        // 11
        push1(),         // 12
        lit(20),         // 13
        jumpi(),         // 14
        push1(),         // 15
        lit(100),        // 16
        push1(),         // 17
        lit(22),         // 18
        jump(),          // 19
        push1(),         // 20
        lit(200),        // 21
        assert_ins(200), // 22
    ];

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

    let sym_results = machine.run_sym();


    assert_eq!(sym_results.pruned.len(), 1);

    assert_eq!(sym_results.pruned.get(0).unwrap().stack.peek().unwrap(), &Word::from(100));

    assert_eq!(sym_results.leaves.len(), 1);

    assert_eq!(sym_results.leaves.get(0).unwrap().stack.peek().unwrap(), &Word::from(200));

}
