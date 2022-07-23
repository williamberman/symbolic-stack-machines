use super::Machine;

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
