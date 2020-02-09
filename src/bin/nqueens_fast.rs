//! n-queens solver

use std::collections::HashMap;
use std::env;
use std::hash::Hash;

pub trait Constraint<V: Eq + Hash, D> {
    fn satisfied(&self, assignment: &HashMap<V, D>) -> bool;
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
    domains: HashMap<V, Vec<D>>,
    constraints: HashMap<V, Vec<C>>,
}

impl<V: Clone + Copy + Eq + Hash, D: Clone + Copy, C: Clone + Constraint<V, D>> CSP<V, D, C> {
    pub fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> CSP<V, D, C> {
        let mut constraints: HashMap<V, Vec<C>> = HashMap::new();
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
    fn consistent(&self, variable: V, assignment: &HashMap<V, D>) -> bool {
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
    pub fn backtracking_search(&self, assignment: HashMap<V, D>) -> Option<HashMap<V, D>> {
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct QueensConstraint(Vec<i8>);
// TODO: use single u64 with bitmask?

impl Constraint<i8, i8> for QueensConstraint {
    fn satisfied(&self, assignment: &HashMap<i8, i8>) -> bool {
        // q1c = queen 1 column, q1r = queen 1 row
        for (q1c, q1r) in assignment.iter() {
            for q2c in (q1c + 1)..(self.0.len() as i8 + 1) {
                if assignment.contains_key(&q2c) {
                    let q2r = assignment.get(&q2c).unwrap();
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
    fn variables(&self) -> Vec<i8> {
        self.0.clone()
    }
}

fn main() {
    let mut args = env::args();
    let n: i8 = match args.nth(1) {
        Some(n) => n.parse::<i8>().unwrap(),
        None => 8,
    };
    // set up N-queens problem
    let vars: Vec<i8> = (0..n).collect();
    let mut domains: HashMap<i8, Vec<i8>> = HashMap::new();
    for v in vars.iter() {
        domains.insert(*v, vars.clone());
    }
    let mut csp: CSP<i8, i8, QueensConstraint> = CSP::new(vars.clone(), domains);

    // add constraints
    csp.add_constraint(QueensConstraint(vars));
    // solve it
    let mut initial_guess: HashMap<i8, i8> = HashMap::new();
    initial_guess.insert(1, 2);
    let solution = csp.backtracking_search(initial_guess);
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
