//! n-queens solver

use std::collections::HashMap;
use std::env;
use std::hash::Hash;

extern crate classic;
use classic::csp::{Constraint, CSP};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct QueensConstraint {
    cols: Vec<i32>,
}

impl Constraint<i32, i32> for QueensConstraint {
    fn satisfied(&self, assignment: &HashMap<i32, i32>) -> bool {
        // q1c = queen 1 column, q1r = queen 1 row
        for (q1c, q1r) in assignment.iter() {
            for q2c in (q1c + 1)..(self.cols.len() as i32 + 1) {
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
    fn variables(&self) -> Vec<i32> {
        self.cols.clone()
    }
}

fn main() {
    let mut args = env::args();
    let n: i32 = match args.nth(1) {
        Some(n) => n.parse::<i32>().unwrap(),
        None => 8,
    };
    // set up N-queens problem
    let vars: Vec<i32> = (0..n).collect();
    let mut domains: HashMap<i32, Vec<i32>> = HashMap::new();
    for v in vars.iter() {
        domains.insert(*v, vars.clone());
    }
    let mut csp: CSP<i32, i32, QueensConstraint> = CSP::new(vars.clone(), domains);

    // add constraints
    csp.add_constraint(QueensConstraint { cols: vars });
    // solve it
    let mut initial_guess: HashMap<i32, i32> = HashMap::new();
    initial_guess.insert(1, 2);
    let solution = csp.backtracking_search(initial_guess);
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
