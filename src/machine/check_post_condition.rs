use std::{collections::HashMap, ops::Not};

use im::Vector;

use crate::{val::constraint::Constraint, z3::solve_z3_all};

use super::Machine;

// Returns true if for any of the machines,
// The path constraints hold and any one of the post conditions are violated
pub fn check_post_condition_violated<Filter, PostCondition>(
    machines: &Vec<Machine>,
    filter: Filter,
    post_condition: PostCondition,
) -> bool
where
    Filter: Fn(&Machine) -> bool,
    PostCondition: Fn(&Machine) -> Vec<Constraint>,
{
    // No machines, none could violate any post condition
    if machines.is_empty() {
        return false;
    }

    let mut rv = false;

    for m in machines {
        if !filter(m) {
            continue;
        }

        let mut constraints = Vector::new();

        post_condition(m).into_iter().for_each(|c| {
            constraints.push_back(c.not());
        });

        m.constraints.iter().for_each(|c| {
            constraints.push_back(c.clone());
        });

        let res = solve_z3_all(&constraints, vec![], vec![], &HashMap::new());

        // There is a solutions such that all path constraints hold and at least one
        // of the post conditions does not hold.
        if let Some(_) = res {
            rv = true;
            break;
        }
    }

    rv
}
