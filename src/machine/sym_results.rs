use crate::z3::solve_z3_all;

use super::Machine;

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
            match solve_z3_all(&m.constraints, vec![], m.calldata.inner().clone()) {
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

#[derive(Debug)]
pub struct SymResults {
    pub leaves: Vec<Machine>,
    pub pruned: Vec<Machine>,
}

impl SymResults {
    pub fn find_reverted(&self, s: String) -> Option<&Machine> {
        let ss = Some(s);
        self.leaves.iter().find(|m| {
            // TODO(will) - should we not have to check solve_results here?
            m.revert_string() == ss && m.solve_results.is_some()
        })
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
