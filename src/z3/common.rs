use std::collections::HashMap;

use primitive_types::U256;
use z3::{
    ast::{Ast, Bool, BV},
    Config, Context, Model,
};

use crate::val::{byte::Byte, constraint::Constraint, word::Word};

static WORD_BITVEC_SIZE: u32 = 256;
static BYTE_BITVEC_SIZE: u32 = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolveResults {
    pub words: HashMap<Word, U256>,
    pub bytes: HashMap<Byte, u8>,
}

pub fn make_z3_config() -> Config {
    let mut cfg = z3::Config::default();
    cfg.set_model_generation(true);
    cfg
}

pub fn make_z3_constraint<'ctx>(
    ctx: &'ctx Context,
    c: &Constraint,
    variables: &HashMap<Word, String>,
) -> Bool<'ctx> {
    match c {
        Constraint::Eq(l, r) => make_z3_bitvec_from_word(ctx, l, variables)
            ._eq(&make_z3_bitvec_from_word(ctx, r, variables)),
        Constraint::Neq(c) => make_z3_constraint(ctx, c, variables).not(),
    }
}

pub fn make_z3_bitvec_from_word<'ctx>(
    ctx: &'ctx Context,
    w: &Word,
    variables: &HashMap<Word, String>,
) -> BV<'ctx> {
    if let Some(variable) = variables.get(w) {
        return BV::new_const(ctx, variable.clone(), WORD_BITVEC_SIZE);
    }

    match w {
        Word::C(c) => {
            // TODO(will) - this looks like it's the current best way to construct a constant
            // 256 BV. The alternative might be using the Int::from_str method and constructing
            // the BV from that Int
            //
            // https://docs.rs/z3/0.11.2/z3/ast/struct.Int.html#method.from_str
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
                .simplify()
        }
        Word::S(x) => BV::new_const(&ctx, x.clone(), WORD_BITVEC_SIZE),
        Word::Add(l, r) => {
            make_z3_bitvec_from_word(ctx, l, variables)
                + make_z3_bitvec_from_word(ctx, r, variables)
        }
        Word::Mul(l, r) => make_z3_bitvec_from_word(ctx, l, variables)
            .bvmul(&make_z3_bitvec_from_word(ctx, r, variables)),
        Word::Sub(l, r) => {
            make_z3_bitvec_from_word(ctx, l, variables)
                - make_z3_bitvec_from_word(ctx, r, variables)
        }
        Word::Div(l, r) => make_z3_bitvec_from_word(ctx, l, variables)
            .bvudiv(&make_z3_bitvec_from_word(ctx, r, variables)),
        Word::Lt(l, r) => bool_to_bitvec(
            ctx,
            make_z3_bitvec_from_word(ctx, l, variables)
                .bvult(&make_z3_bitvec_from_word(ctx, r, variables)),
        ),
        Word::LtEq(l, r) => bool_to_bitvec(
            ctx,
            make_z3_bitvec_from_word(ctx, l, variables)
                .bvule(&make_z3_bitvec_from_word(ctx, r, variables)),
        ),
        Word::Gt(l, r) => bool_to_bitvec(
            ctx,
            make_z3_bitvec_from_word(ctx, l, variables)
                .bvugt(&make_z3_bitvec_from_word(ctx, r, variables)),
        ),
        Word::Slt(l, r) => bool_to_bitvec(
            ctx,
            make_z3_bitvec_from_word(ctx, l, variables)
                .bvslt(&make_z3_bitvec_from_word(ctx, r, variables)),
        ),
        Word::Shr(value, shift) => make_z3_bitvec_from_word(ctx, value, variables)
            .bvlshr(&make_z3_bitvec_from_word(ctx, shift, variables)),
        Word::BitAnd(l, r) => make_z3_bitvec_from_word(ctx, l, variables)
            .bvand(&make_z3_bitvec_from_word(ctx, r, variables)),
        Word::BitOr(l, r) => make_z3_bitvec_from_word(ctx, l, variables)
            .bvor(&make_z3_bitvec_from_word(ctx, r, variables)),
        Word::Ite(q, then, xelse) => make_z3_constraint(ctx, q, variables).ite(
            &make_z3_bitvec_from_word(ctx, then, variables),
            &make_z3_bitvec_from_word(ctx, xelse, variables),
        ),
        Word::Concat(x) => x
            .iter()
            .map(|b| make_z3_bitvec_from_byte(ctx, b, variables))
            .reduce(|l, r| l.concat(&r))
            .unwrap(),
    }
}

pub fn make_z3_bitvec_from_byte<'ctx>(
    ctx: &'ctx Context,
    b: &Byte,
    variables: &HashMap<Word, String>,
) -> BV<'ctx> {
    match b {
        Byte::C(x, _) => BV::from_u64(ctx, *x as u64, BYTE_BITVEC_SIZE),
        Byte::S(x) => BV::new_const(&ctx, x.clone(), BYTE_BITVEC_SIZE),
        Byte::Idx(word, idx) => {
            let indices = ByteIndices::from(*idx);

            BV::extract(
                &make_z3_bitvec_from_word(ctx, word, variables),
                indices.high,
                indices.low,
            )
        }
    }
}

pub fn bool_to_bitvec<'ctx>(ctx: &'ctx Context, bool: Bool<'ctx>) -> BV<'ctx> {
    bool.ite(
        &BV::from_u64(ctx, 1, WORD_BITVEC_SIZE),
        &BV::from_u64(ctx, 0, WORD_BITVEC_SIZE),
    )
}

pub fn make_solve_results<'ctx>(
    model: Model,
    words: Vec<(Word, BV<'ctx>)>,
    bytes: Vec<(Byte, BV<'ctx>)>,
) -> SolveResults {
    let mut word_results: HashMap<Word, U256> = HashMap::new();

    words.iter().for_each(|(w, bv)| {
        // TODO Handle larger than u64
        let word_result = model.eval(bv, true).unwrap().as_u64().unwrap();
        word_results.insert(w.clone(), word_result.into());
    });

    let mut byte_results = HashMap::new();

    bytes.iter().for_each(|(b, bv)| {
        let byte_result = model.eval(bv, true).unwrap().as_u64().unwrap();

        byte_results.insert(b.clone(), byte_result as u8);
    });

    SolveResults {
        words: word_results,
        bytes: byte_results,
    }
}

// Handles the conversion from byte index into indices necessary for BV::extract
// see `test_concat_order` and `test_byte_extraction` for more details
struct ByteIndices {
    low: u32,
    high: u32,
}

impl From<usize> for ByteIndices {
    fn from(array_idx: usize) -> Self {
        let high = 255 - 8 * array_idx;
        let low = high - 7;

        Self {
            low: low.try_into().unwrap(),
            high: high.try_into().unwrap(),
        }
    }
}

mod tests {
    use std::collections::HashMap;

    use z3::ast::{Ast, BV};

    #[allow(dead_code)]
    static BS: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    #[test]
    fn test_byte_extraction() {
        let cfg = super::make_z3_config();
        let ctx = z3::Context::new(&cfg);

        let w = crate::val::word::Word::Concat(BS.map(|x| x.into()));

        for i in 0..=31 {
            let byte = crate::val::byte::Byte::Idx(Box::new(w.clone()), i);

            let bv_byte = super::make_z3_bitvec_from_byte(&ctx, &byte, &HashMap::new()).simplify();

            let extracted_byte = bv_byte.as_u64().unwrap() as usize;

            assert_eq!(i, extracted_byte);
        }
    }

    #[test]
    // The low byte of an array converted into a Z3 Bit vector ends up
    // being indexed as the high byte and vice versa.
    fn test_concat_order() {
        let cfg = super::make_z3_config();
        let ctx = z3::Context::new(&cfg);

        let w = crate::val::word::Word::Concat(BS.map(|x| x.into()));

        // #x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f
        let bv = super::make_z3_bitvec_from_word(&ctx, &w, &HashMap::new()).simplify();

        // #x1f
        let high_byte_extracted = BV::extract(&bv, 7, 0).simplify();

        assert_eq!(high_byte_extracted.as_u64().unwrap(), 31);

        // #x00
        let low_byte_extracted = BV::extract(&bv, 255, 248).simplify();

        assert_eq!(high_byte_extracted.as_u64().unwrap(), 31);
        assert_eq!(low_byte_extracted.as_u64().unwrap(), 0);
    }
}
