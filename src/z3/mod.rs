use std::collections::HashMap;

use im::Vector;
use primitive_types::U256;
use z3::{
    ast::{Ast, Bool, BV},
    Context, SatResult,
};

use crate::val::{byte::Byte, constraint::Constraint, word::Word};

static WORD_BITVEC_SIZE: u32 = 256;
static BYTE_BITVEC_SIZE: u32 = 8;

#[derive(Debug, PartialEq, Eq)]
pub struct SolveResults {
    pub words: HashMap<Word, U256>,
    pub bytes: HashMap<Byte, u8>,
}

pub fn solve_z3(
    constraints: &Vector<Constraint>,
    words: Vec<Word>,
    bytes: Vec<Byte>,
) -> Option<SolveResults> {
    let mut cfg = z3::Config::default();
    cfg.set_model_generation(true);
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    constraints.iter().for_each(|c| {
        solver.assert(&make_constraint(&ctx, c));
    });

    if solver.check() != SatResult::Sat {
        return None;
    };

    let model = solver.get_model().unwrap();

    let mut word_results: HashMap<Word, U256> = HashMap::new();

    words.iter().for_each(|w| {
        // TODO Handle larger than u64
        let word_result = model
            .eval(&make_bitvec_from_word(&ctx, w), true)
            .unwrap()
            .as_u64()
            .unwrap();
        word_results.insert(w.clone(), word_result.into());
    });

    let mut byte_results = HashMap::new();

    bytes.iter().for_each(|b| {
        let byte_result = model
            .eval(&make_bitvec_from_byte(&ctx, b), true)
            .unwrap()
            .as_u64()
            .unwrap();

        byte_results.insert(b.clone(), byte_result as u8);
    });

    Some(SolveResults {
        words: word_results,
        bytes: byte_results,
    })
}

pub fn make_constraint<'ctx>(ctx: &'ctx Context, c: &Constraint) -> Bool<'ctx> {
    match c {
        Constraint::Eq(l, r) => make_bitvec_from_word(ctx, l)._eq(&make_bitvec_from_word(ctx, r)),
        Constraint::Neq(c) => make_constraint(ctx, c).not(),
    }
}

pub fn make_bitvec_from_word<'ctx>(ctx: &'ctx Context, w: &Word) -> BV<'ctx> {
    match w {
        // TODO needs to support larger sizes than u64
        Word::C(x) => BV::from_u64(ctx, x.as_u64(), WORD_BITVEC_SIZE),
        Word::S(x) => BV::new_const(&ctx, x.clone(), WORD_BITVEC_SIZE),
        Word::Add(l, r) => make_bitvec_from_word(ctx, l) + make_bitvec_from_word(ctx, r),
        Word::Sub(l, r) => make_bitvec_from_word(ctx, l) - make_bitvec_from_word(ctx, r),
        Word::Lt(l, r) => make_bitvec_from_word(ctx, l)
            .bvult(&make_bitvec_from_word(ctx, r))
            .ite(
                &BV::from_u64(ctx, 1, WORD_BITVEC_SIZE),
                &BV::from_u64(ctx, 0, WORD_BITVEC_SIZE),
            ),
        Word::Ite(q, then, xelse) => make_constraint(ctx, q).ite(
            &make_bitvec_from_word(ctx, then),
            &make_bitvec_from_word(ctx, xelse),
        ),
        Word::Concat(x) => x
            .iter()
            .map(|b| make_bitvec_from_byte(ctx, b))
            .reduce(|l, r| l.concat(&r))
            .unwrap(),
    }
}

pub fn make_bitvec_from_byte<'ctx>(ctx: &'ctx Context, b: &Byte) -> BV<'ctx> {
    match b {
        Byte::C(x) => BV::from_u64(ctx, *x as u64, BYTE_BITVEC_SIZE),
        Byte::S(x) => BV::new_const(&ctx, x.clone(), BYTE_BITVEC_SIZE),
    }
}
