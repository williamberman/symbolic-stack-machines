use super::{assertions::DEFAULT_ASSERTIONS, sym_results::SymResults, Machine};

impl Machine {
    pub fn run_sym(self) -> SymResults {
        let complete = self.run_sym_inner();

        let mut leaves = vec![];
        let mut pruned = vec![];

        complete
            .into_iter()
            .for_each(|mut m| match m.solve_z3_all(None) {
                Some(solve_results) => {
                    m.solve_results = Some(solve_results);
                    leaves.push(m);
                }
                None => pruned.push(m),
            });

        SymResults { leaves, pruned }
    }

    pub fn run_sym_check_assertions(self, assertions: Option<Vec<&str>>) -> SymResults {
        let complete = self.run_sym_inner();

        let mut leaves: Vec<Machine> = vec![];
        let mut pruned: Vec<Machine> = vec![];

        let assertions = match assertions {
            Some(assertions) => assertions,
            None => DEFAULT_ASSERTIONS.to_vec(),
        };

        complete
            .into_iter()
            .for_each(|mut m| match m.revert_string() {
                Some(r) => {
                    if assertions.contains(&r.as_str()) {
                        match m.solve_z3_all(None) {
                            Some(solve_results) => {
                                m.solve_results = Some(solve_results);
                                leaves.push(m);
                            }
                            None => pruned.push(m),
                        }
                    } else {
                        pruned.push(m);
                    }
                }
                None => pruned.push(m),
            });

        SymResults { leaves, pruned }
    }

    fn run_sym_inner(self) -> Vec<Machine> {
        let mut queue: Vec<Machine> = vec![self];
        let mut complete: Vec<Machine> = vec![];

        loop {
            match queue.pop() {
                Some(m) => {
                    if m.halt {
                        complete.push(m);
                    } else {
                        let (new_machine, branch) = m.step_sym();

                        queue.push(new_machine);

                        if let Some(branch) = branch {
                            queue.push(branch);
                        }
                    }
                }
                None => break,
            }
        }

        complete
    }
}
