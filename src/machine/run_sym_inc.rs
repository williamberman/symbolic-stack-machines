use super::{sym_results::SymResults, Machine};
use crate::z3::{make_solve_results, make_z3_bitvec_from_word, make_z3_bitvec_from_byte};
use crate::z3::{make_z3_config, make_z3_constraint};
use log::info;
use std::time::Instant;
use z3::SatResult;

impl Machine {
    pub fn run_sym_inc(self) -> SymResults {
        let cfg = make_z3_config();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);

        let mut cur: Option<Machine> = Some(self);
        let mut work_stack: Vec<Machine> = vec![];

        let mut pruned: Vec<Machine> = vec![];
        let mut leaves: Vec<Machine> = vec![];

        loop {
            match cur {
                Some(m) => {
                    if m.halt {
                        leaves.push(m);
                        cur = None;
                        solver.pop(1);
                    } else {
                        let (new_machine, branch) = m.step_sym();

                        match branch {
                            None => {
                                // No new constraints added
                                cur = Some(new_machine);
                            }

                            // New constraints added, on next loop try to take branches
                            Some(branch) => {
                                cur = None;
                                work_stack.push(new_machine);
                                work_stack.push(branch);
                            }
                        }
                    }
                }
                None => {
                    match work_stack.pop() {
                        Some(mut m) => {
                            solver.push();

                            let z3_constraint = make_z3_constraint(
                                &ctx,
                                m.constraints.get(m.constraints.len() - 1).unwrap(),
                                &None
                            );

                            solver.assert(&z3_constraint);

                            let timer = Instant::now();

                            info!("solving num_constaints: {}", m.constraints.len(),);

                            let solver_res = solver.check();

                            let elapsed = timer.elapsed();

                            info!("time elapsed: {:.2?}, result: {:?}", elapsed, solver_res);

                            if solver_res != SatResult::Sat {
                                solver.pop(1);
                                pruned.push(m);
                            } else {
                                let model = solver.get_model().unwrap();
                                let solve_results = make_solve_results(
                                    model,
                                    vec![],
                                    m.calldata.inner().clone().into_iter().map(|b| {
                                        let bv = make_z3_bitvec_from_byte(&ctx, &b, &None);
                                        (b, bv)
                                    }).collect(),
                                );
                                m.solve_results = Some(solve_results);
                                cur = Some(m);
                            }
                        }

                        // There is no current machine to step and the work stack is empty.
                        // Exit the loop
                        None => break,
                    }
                }
            }

            let work_stack_len_add = match cur {
                Some(_) => 1,
                None => 0,
            };

            info!(
                "work_stack: {}, leaves: {}, pruned: {}",
                work_stack.len() + work_stack_len_add,
                leaves.len(),
                pruned.len()
            );
        }

        SymResults { leaves, pruned }
    }
}
