use im::Vector;
use log::info;
use std::time::Instant;
use z3::ast::{Ast, BV};
use z3::SatResult;

use crate::val::{byte::Byte, constraint::Constraint, word::Word};
use crate::z3::common::{
    make_solve_results, make_z3_bitvec_from_byte, make_z3_config, make_z3_constraint,
};
use crate::z3::make_z3_bitvec_from_word;
use crate::z3::script_writer::Smtlib2ScriptFileWriter;

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

    let mut script_writer = if DUMP_CONSTRAINTS {
        Some(Smtlib2ScriptFileWriter::new())
    } else {
        None
    };

    let bytes: Vec<(Byte, BV)> = bytes
        .into_iter()
        .map(|b| {
            let bv = make_z3_bitvec_from_byte(&ctx, &b, &None);

            // Only write symbolic bytes
            match &b {
                Byte::S(_) => {
                    if let Some(script_writer) = &mut script_writer {
                        script_writer.write_byte(&bv);
                    }
                }
                _ => {}
            }

            (b, bv)
        })
        .collect();

    if let Some(script_writer) = &mut script_writer {
        script_writer.write_newline();
    }

    let words: Vec<(Word, BV)> = words
        .into_iter()
        .map(|w| {
            let bv = make_z3_bitvec_from_word(&ctx, &w, &None);
            if let Some(script_writer) = &mut script_writer {
                script_writer.write_word(&bv);
            }
            (w, bv)
        })
        .collect();

    if let Some(script_writer) = &mut script_writer {
        script_writer.write_newline();
    }

    constraints.iter().for_each(|c| {
        let z3_constraint = make_z3_constraint(&ctx, c, &None).simplify();
        solver.assert(&z3_constraint);
    });

    if let Some(script_writer) = &mut script_writer {
        constraints.iter().for_each(|c| {
            let z3_constraint = make_z3_constraint(&ctx, c, &None).simplify();
            script_writer.write_constraint(&z3_constraint);
        });
    }

    let timer = Instant::now();

    if let Some(script_writer) = script_writer {
        info!(
            "solving num_constaints: {}, constraints written to: {}",
            constraints.len(),
            script_writer.file_path.to_str().unwrap()
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

    Some(make_solve_results(model, words, bytes))
}
