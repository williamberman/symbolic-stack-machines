use log::info;

use super::{sym_results::SymResults, Machine};

impl Machine {
    pub fn run_sym_solve_at_each_branch(self) -> SymResults {
        let mut rv = SymResultsWithSolver::new(self);

        loop {
            let start_branch = rv.queue.pop();
            if let Some(mach) = start_branch {
                if !mach.halt {
                    let n_constraints = mach.constraints.len();
                    let (new_machine, branch) = mach.step_sym();

                    let mut new_machines = vec![new_machine];

                    if let Some(branch) = branch {
                        new_machines.push(branch);
                    }

                    new_machines.into_iter().for_each(|m| {
                        // Do not constraint solve when number constraints doesn't change
                        // because constraints can only be added
                        let cs = m.constraint_solve && m.constraints.len() != n_constraints;
                        rv.push(m, cs);
                    });
                } else {
                    rv.leaves.push(mach);
                }
            } else {
                break;
            }

            info!(
                "queue: {}, leaves: {}, pruned: {}",
                rv.queue.len(),
                rv.leaves.len(),
                rv.pruned.len()
            );
        }

        rv.into()
    }
}

#[derive(Debug)]
pub struct SymResultsWithSolver {
    pub queue: Vec<Machine>,
    pub leaves: Vec<Machine>,
    pub pruned: Vec<Machine>,
}

impl SymResultsWithSolver {
    pub fn new(m: Machine) -> Self {
        Self {
            queue: vec![m],
            leaves: vec![],
            pruned: vec![],
        }
    }

    pub fn push(&mut self, mut m: Machine, constraint_solve: bool) {
        if constraint_solve && !m.constraints.is_empty() {
            match m.solve_z3_all(None) {
                Some(sr) => {
                    m.solve_results = Some(sr);
                    self.push_inner(m);
                }
                None => self.pruned.push(m),
            }
        } else {
            self.push_inner(m)
        }
    }

    fn push_inner(&mut self, m: Machine) {
        if m.halt {
            self.leaves.push(m)
        } else {
            self.queue.push(m)
        }
    }
}

impl Into<SymResults> for SymResultsWithSolver {
    fn into(self) -> SymResults {
        SymResults {
            leaves: self.leaves,
            pruned: self.pruned,
        }
    }
}
