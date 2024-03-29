//! Constraint Satisfaction Problems
//! Template types plus backtracking solver

//use std::collections::HashMap;
use std::hash::Hash;
use fnv::FnvHashMap;

pub trait Constraint<V: Eq + Hash, D> {
    fn satisfied(&self, assignment: &FnvHashMap<V, D>) -> bool;
    fn variables(&self) -> Vec<V>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CSP<V, D, C>
where
    V: Clone + Copy + Eq + Hash,
    D: Clone + Copy,
    C: Clone + Constraint<V, D>,
{
    variables: Vec<V>,
    domains: FnvHashMap<V, Vec<D>>,
    constraints: FnvHashMap<V, Vec<C>>,
}

impl<V: Clone + Copy + Eq + Hash, D: Clone + Copy, C: Clone + Constraint<V, D>> CSP<V, D, C> {
    pub fn new(variables: Vec<V>, domains: FnvHashMap<V, Vec<D>>) -> CSP<V, D, C> {
        let mut constraints: FnvHashMap<V, Vec<C>> = FnvHashMap::default();
        for variable in &variables {
            constraints.insert(*variable, Vec::new());
            if !domains.contains_key(&variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        CSP {
            variables,
            domains,
            constraints,
        }
    }
    pub fn add_constraint(&mut self, constraint: C) {
        let vars = constraint.variables();
        for variable in vars {
            if !self.variables.contains(&variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                self.constraints
                    .get_mut(&variable)
                    .unwrap()
                    .push(constraint.clone())
            }
        }
    }
    fn consistent(&self, variable: V, assignment: &FnvHashMap<V, D>) -> bool {
        let constraints = self.constraints.get(&variable).unwrap();
        for c in constraints {
            if !c.satisfied(assignment) {
                return false;
            }
        }
        true
        //.iter()
        //.any(|&c| !*c.satisfied(*assignment))
    }
    pub fn backtracking_search(&self, assignment: FnvHashMap<V, D>) -> Option<FnvHashMap<V, D>> {
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }
        let unassigned: Vec<V> = self
            .variables
            .clone()
            .into_iter()
            .filter(|&v| !assignment.contains_key(&v))
            .collect();
        let first = unassigned[0];
        for value in &self.domains[&first] {
            let mut local_assignment = assignment.clone();
            local_assignment.insert(first, *value);
            if self.consistent(first, &local_assignment) {
                if let Some(result) = self.backtracking_search(local_assignment) {
                    return Some(result);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        ()
    }
}
