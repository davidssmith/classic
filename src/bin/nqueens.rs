//! n-queens solver

use std::hash::Hash;
use std::collections::HashMap;
extern crate classic;
use classic::csp::{CSP, Constraint};


#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct QueensConstraint {
    cols: Vec<i32>,
}

impl Constraint<i32,i32> for QueensConstraint {
    fn satisfied(&self, assignment: &HashMap<i32,i32>) -> bool {
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
    let cols: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7 ,8];
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
    for c in cols.iter() {
        rows.insert(*c, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    let csp: CSP<i32,i32,QueensConstraint> = CSP::new(cols, rows);
    let mut initial_guess: HashMap<i32, i32> = HashMap::new();
    initial_guess.insert(1, 1);
    let solution = csp.backtracking_search(initial_guess);
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
