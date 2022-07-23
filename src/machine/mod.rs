pub mod assertions;
pub mod mem_ptr;
pub mod check_post_condition;
mod run_sym_inc;
mod run_sym_solve_at_each_branch;
mod run_sym_solve_at_end;
mod sym_results;

use std::{
    collections::HashMap,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use im::Vector;

use crate::{
    calldata::Calldata,
    instructions::{Instruction, InstructionResult},
    memory::Memory,
    stack::Stack,
    val::{byte::Byte, constraint::Constraint, word::Word},
    z3::SolveResults,
};

use self::{mem_ptr::MemPtr, sym_results::SymResults};

#[derive(Debug)]
pub struct Machine {
    pub id: usize,
    pub stack: Stack,
    pub mem: Memory,
    pub pc: usize,
    pub pgm: Rc<Vec<Instruction>>,
    pub calldata: Rc<Calldata>,
    pub constraints: Vector<Constraint>,
    pub halt: bool,
    pub call_value: Word,
    pub return_ptr: Option<MemPtr>,
    pub revert_ptr: Option<MemPtr>,

    pub constraint_solve: bool,
    pub ctr: Arc<AtomicUsize>,
    pub solve_results: Option<SolveResults>,
}

impl Clone for Machine {
    fn clone(&self) -> Self {
        let new_id = self.ctr.fetch_add(1, Ordering::SeqCst);

        Self {
            id: new_id,
            stack: self.stack.clone(),
            mem: self.mem.clone(),
            pc: self.pc.clone(),
            pgm: self.pgm.clone(),
            calldata: self.calldata.clone(),
            constraints: self.constraints.clone(),
            halt: self.halt.clone(),
            call_value: self.call_value.clone(),
            return_ptr: self.return_ptr.clone(),
            revert_ptr: self.revert_ptr.clone(),

            constraint_solve: self.constraint_solve,
            ctr: self.ctr.clone(),
            solve_results: self.solve_results.clone(),
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            id: Default::default(),
            stack: Default::default(),
            mem: Default::default(),
            pc: 0,
            pgm: Default::default(),
            calldata: Default::default(),
            constraints: Default::default(),
            halt: false,
            call_value: Default::default(),
            return_ptr: Default::default(),
            revert_ptr: Default::default(),

            constraint_solve: true,
            ctr: Arc::new(AtomicUsize::new(1)),
            solve_results: Default::default(),
        }
    }
}

impl Machine {
    pub fn new(pgm: Vec<Instruction>) -> Self {
        let mut m = Self::default();
        m.pgm = Rc::new(pgm);
        m
    }

    pub fn run(self) -> Machine {
        let mut x = self;

        while !x.halt {
            x = x.step();
        }

        x
    }

    pub fn run_sym(self, assertions: Option<Vec<&str>>) -> SymResults {
        self.run_sym_solve_at_end(assertions)
    }

    pub fn step(self) -> Machine {
        let i = self.pgm.get(self.pc).unwrap().clone();

        // Assume only one is returned
        i.exec(self).0
    }

    pub fn step_sym(self) -> InstructionResult {
        let i = self.pgm.get(self.pc).unwrap().clone();

        i.exec(self)
    }

    pub fn revert_bytes(&self) -> Option<Vec<Byte>> {
        self.revert_ptr.clone().map(|ptr| self.mem_ptr_bytes(ptr))
    }

    pub fn revert_string(&self) -> Option<String> {
        self.revert_ptr.clone().map(|ptr| self.mem_ptr_string(ptr))
    }

    pub fn return_bytes(&self) -> Option<Vec<Byte>> {
        self.return_ptr.clone().map(|ptr| self.mem_ptr_bytes(ptr))
    }

    pub fn return_word(&self) -> Option<Word> {
        self.return_bytes().map(|bs| {
            let arr: [Byte; 32] = bs.try_into().unwrap();
            Word::from(arr)
        })
    }

    pub fn return_string(&self) -> Option<String> {
        self.return_ptr.clone().map(|ptr| self.mem_ptr_string(ptr))
    }

    fn mem_ptr_bytes(&self, ptr: MemPtr) -> Vec<Byte> {
        self.mem.read_bytes(ptr.offset, ptr.length)
    }

    fn mem_ptr_string(&self, ptr: MemPtr) -> String {
        let bytes = self.mem_ptr_bytes(ptr);
        let bs: Vec<u8> = bytes.into_iter().map(|x| x.into()).collect();
        hex::encode(bs)
    }

    fn variables(&self) -> HashMap<Word, String> {
        self.calldata.variables_word_lookup()
    }

    pub fn returned(&self) -> bool {
        self.return_ptr.is_some()
    }
}
