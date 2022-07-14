use primitive_types::U256;
use symbolic_stack_machines::val::{constraint::Constraint, word::Word};

#[test]
pub fn test_nested_ite_term_rewrite_nop() {
    let constraint: Constraint = Constraint::Eq(
        Box::new(Word::Ite(
            Box::new(Word::C(U256::from(1337_u32))._eq(Word::S("leet".into()))),
            Box::new(Word::zero()),
            Box::new(Word::one()),
        )),
        Box::new(Word::zero()),
    );

    let optimized = constraint.ite(Word::zero(), Word::one());

    assert_eq!(
        optimized,
        Word::Ite(
            Box::new(Word::C(U256::from(1337_u32))._eq(Word::S("leet".into()))),
            Box::new(Word::zero()),
            Box::new(Word::one()),
        )
    )
}

#[test]
pub fn test_nested_ite_term_rewrite_flip_then_else() {
    let constraint: Constraint = Constraint::Eq(
        Box::new(Word::Ite(
            Box::new(Word::C(U256::from(1337_u32))._eq(Word::S("leet".into()))),
            Box::new(Word::zero()),
            Box::new(Word::one()),
        )),
        Box::new(Word::zero()),
    );

    let optimized = constraint.ite(Word::one(), Word::zero());

    assert_eq!(
        optimized,
        Word::Ite(
            Box::new(Word::C(U256::from(1337_u32))._eq(Word::S("leet".into()))),
            Box::new(Word::one()),
            Box::new(Word::zero()),
        )
    )
}

#[test]
pub fn test_nested_ite_term_rewrite_flip_then_else_2() {
    let constraint: Constraint = Constraint::Eq(
        Box::new(Word::Ite(
            Box::new(Word::C(U256::from(1337_u32))._eq(Word::S("leet".into()))),
            Box::new(Word::zero()),
            Box::new(Word::one()),
        )),
        Box::new(Word::one()),
    );

    let optimized = constraint.ite(Word::zero(), Word::one());

    assert_eq!(
        optimized,
        Word::Ite(
            Box::new(Word::C(U256::from(1337_u32))._eq(Word::S("leet".into()))),
            Box::new(Word::one()),
            Box::new(Word::zero()),
        )
    )
}
