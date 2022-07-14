use im::Vector;
use log::info;
use z3::ast::Ast;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use uuid::Uuid;
use z3::SatResult;

use crate::val::{byte::Byte, constraint::Constraint, word::Word};
use crate::z3::common::{make_z3_config, make_z3_constraint, make_solve_results};

use super::SolveResults;

static DUMP_CONSTRAINTS: bool = true;

pub fn solve_z3_all(
    constraints: &Vector<Constraint>,
    words: Vec<Word>,
    bytes: Vec<Byte>,
) -> Option<SolveResults> {
    let cfg = make_z3_config();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let constraint_dump_file = if DUMP_CONSTRAINTS {
        let file_name = format!("{}.smtlib2", Uuid::new_v4().to_string());
        let file_path = std::env::temp_dir().join(file_name);
        let f = File::create(file_path.clone()).unwrap();
        Some((f, file_path))
    } else {
        None
    };

    constraints.iter().for_each(|c| {
        let z3_constraint = make_z3_constraint(&ctx, c);
        let z3_constraint_simplified = z3_constraint.simplify();
        if let Some(mut f) = &constraint_dump_file.as_ref().map(|x| &x.0) {
            let s = z3_constraint_simplified.to_string();
            f.write(s.as_bytes()).unwrap();
            // TODO(will): rust platform agnostic newline?
            f.write("\n\n".as_bytes()).unwrap();
        };
        solver.assert(&z3_constraint_simplified);
    });

    let timer = Instant::now();

    if let Some((_, file_path)) = constraint_dump_file {
        info!(
            "solving num_constaints: {}, constraints written to: {}",
            constraints.len(),
            file_path.to_str().unwrap()
        );
    } else {
        info!("solving num_constaints: {}", constraints.len());
    };

    let solver_res = solver.check();

    let elapsed = timer.elapsed();

    info!("time elapsed: {:.2?}, result: {:?}", elapsed, solver_res);

    if solver_res != SatResult::Sat {
        return None;
    };

    let model = solver.get_model().unwrap();

    Some(make_solve_results(&ctx, model, words, bytes))
}
