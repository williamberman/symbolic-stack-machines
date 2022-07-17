use primitive_types::U256;
use symbolic_stack_machines::{
    val::word::Word,
    z3::{make_z3_bitvec_from_word, make_z3_config},
};
use z3::SatResult;

#[test]
pub fn test_constant_conversion() {
    let cfg = make_z3_config();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let w: Word = U256::from([
        01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31, 32,
    ])
    .into();

    let bv = make_z3_bitvec_from_word(&ctx, &w, &None);

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model().unwrap();

    let res = model.eval(&bv, true).unwrap();

    assert_eq!(
        res.to_string(),
        "#x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20"
    );
}
