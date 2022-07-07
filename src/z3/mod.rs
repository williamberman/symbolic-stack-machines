use std::collections::HashMap;

use im::Vector;
use primitive_types::U256;
use z3::{
    ast::{Ast, Bool, BV},
    Context, SatResult, Config,
};

use crate::val::{byte::Byte, constraint::Constraint, word::Word};

static WORD_BITVEC_SIZE: u32 = 256;
static BYTE_BITVEC_SIZE: u32 = 8;

#[derive(Debug, PartialEq, Eq)]
pub struct SolveResults {
    pub words: HashMap<Word, U256>,
    pub bytes: HashMap<Byte, u8>,
}

pub fn make_z3_config() -> Config {
    let mut cfg = z3::Config::default();
    cfg.set_model_generation(true);
    cfg
}

pub fn solve_z3(
    constraints: &Vector<Constraint>,
    words: Vec<Word>,
    bytes: Vec<Byte>,
) -> Option<SolveResults> {
    let cfg = make_z3_config();
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
        Word::C(c) => {
            let mut bytes = vec![0; 32];

            c.to_big_endian(&mut bytes);

            let x: [u8; 8] = bytes[0..8].try_into().unwrap();
            let y: [u8; 8] = bytes[8..16].try_into().unwrap();
            let z: [u8; 8] = bytes[16..24].try_into().unwrap();
            let w: [u8; 8] = bytes[24..32].try_into().unwrap();

            BV::from_u64(ctx, u64::from_be_bytes(x), 64)
                .concat(&BV::from_u64(ctx, u64::from_be_bytes(y), 64))
                .concat(&BV::from_u64(ctx, u64::from_be_bytes(z), 64))
                .concat(&BV::from_u64(ctx, u64::from_be_bytes(w), 64))
        }
        Word::S(x) => BV::new_const(&ctx, x.clone(), WORD_BITVEC_SIZE),
        Word::Add(l, r) => make_bitvec_from_word(ctx, l) + make_bitvec_from_word(ctx, r),
        Word::Mul(l, r) => make_bitvec_from_word(ctx, l).bvmul(&make_bitvec_from_word(ctx, r)),
        Word::Sub(l, r) => make_bitvec_from_word(ctx, l) - make_bitvec_from_word(ctx, r),
        Word::Div(l, r) => make_bitvec_from_word(ctx, l).bvudiv(&make_bitvec_from_word(ctx, r)),
        Word::Lt(l, r) => bool_to_bitvec(
            ctx,
            make_bitvec_from_word(ctx, l).bvult(&make_bitvec_from_word(ctx, r)),
        ),
        Word::Gt(l, r) => bool_to_bitvec(
            ctx,
            make_bitvec_from_word(ctx, l).bvugt(&make_bitvec_from_word(ctx, r)),
        ),
        Word::Slt(l, r) => bool_to_bitvec(
            ctx,
            make_bitvec_from_word(ctx, l).bvslt(&make_bitvec_from_word(ctx, r)),
        ),
        Word::Shr(value, shift) => {
            make_bitvec_from_word(ctx, value).bvlshr(&make_bitvec_from_word(ctx, shift))
        }
        Word::BitAnd(l, r) => make_bitvec_from_word(ctx, l).bvand(&make_bitvec_from_word(ctx, r)),
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

pub fn bool_to_bitvec<'ctx>(ctx: &'ctx Context, bool: Bool<'ctx>) -> BV<'ctx> {
    bool.ite(
        &BV::from_u64(ctx, 1, WORD_BITVEC_SIZE),
        &BV::from_u64(ctx, 0, WORD_BITVEC_SIZE),
    )
}
