//! n-queens solver

use std::hash::Hash;
use std::collections::HashMap;

pub trait Constraint<V: Hash, D> {
    fn satisfied(&self, assignment: HashMap<V,D>) -> bool;
}

// A constraint satisfaction problem consists of variables of type V
// that have ranges of values known as domains of type D and constraints
// that determine whether a particular variable's domain selection is valid
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CSP<V: Hash + Eq, D: Hash> {
    variables: Vec<V>,
    domains: HashMap<V, Vec<D>>,
    constraints: HashMap<V, Vec<D>>,
}


impl<V: Hash + Eq, D: Hash> CSP<V,D> {
    fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> CSP<V,D> {
        let mut constraints: HashMap<V,Vec<dyn Constraint<V,D>>> = HashMap::new();
        for variable in variables {
            constraints.insert(variable, Vec::new());
            if !self.domains.contains_key(variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        CSP { variables, domains, constraints }
    }
    fn add_constraint<T: Constraint<V,D>>(&mut self, constraint: T) {
        for variable in constraint.variables.iter() {
            if !self.variables.contains_key(variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                self.constraints[variable].push(constraint);
            }
        }
    }

    // Check if the value assignment is consistent by checking all constraints
    // for the given variable against it
    fn consistent(&self, variable: V, assignment: HashMap<V,D>) -> bool {
        for constraint in self.constraints.get(&variable) {
            if !constraint.satisfied(assignment) {
                return false;
            }
        }
        true
    }

    fn backtracking_search(&self, assignment: HashMap<V,D>) -> Option<HashMap<V,D>> {
        // assignment is complete if every variable is assigned (our base case)
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }
        // get all variables in the CSP but not in the assignment
        let unassigned: Vec<V> = self.variables.iter()
            .filter(|v| !assignment.contains_key(v))
            .collect();

        // get the every possible do    main value of the first unassigned variable
        let first: V = unassigned[0];
        for value in self.domains[first] {
            let local_assignment = assignment.clone();
            local_assignment[first] = value;
            // if we're still consistent, we recurse (continue)
            if self.consistent(first, local_assignment) {
                let result = self.backtracking_search(local_assignment);
                // if we didn't find the result, we will end up backtracking
                if let Some(result) {
                    return result.unwrap();
                }
            }
        }
        None
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct QueensConstraint {
    cols: Vec<i32>,
}

impl<V: Hash + Eq, D: Eq> Constraint<V,D> for QueensConstraint {
    fn satisfied(&self, assignment: HashMap<V, D>) -> bool {
        // q1c = queen 1 column, q1r = queen 1 row
        for (q1c, q1r) in assignment.iter() {
            for q2c in (q1c + 1)..(self.cols.len() + 1) {
                if assignment.contains_key(q2c) {
                    let q2r = assignment.get(&q2c);
                    if q1r == q2r {
                        return false;
                    } else if (q1r - q2r).abs() == (q1c - q2c).abs() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn main() {
    let cols: [i32; 8] = [1, 2, 3, 4, 5, 6, 7 ,8];
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
    for &mut c in cols.iter_mut() {
        rows.insert(c, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    let csp: CSP<i32,i32> = CSP::new(cols, rows);
}
