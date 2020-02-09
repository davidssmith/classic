//! n-queens solver

//use std::env;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

extern crate classic;
use classic::csp::{Constraint, CSP};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct SendMoneyConstraint(Vec<char>);

impl Constraint<char, i8> for SendMoneyConstraint {
    fn satisfied(&self, assignment: &FnvHashMap<char, i8>) -> bool {
        // if there are duplicate values then it's not a solution
        if assignment.values().unique().count() < assignment.len() {
            return false;
        }

        // if all variables have been assigned, check if it adds correctly
        if assignment.len() == self.0.len() {
            let s: i32 = assignment[&'S'] as i32;
            let e: i32 = assignment[&'E'] as i32;
            let n: i32 = assignment[&'N'] as i32;
            let d: i32 = assignment[&'D'] as i32;
            let m: i32 = assignment[&'M'] as i32;
            let o: i32 = assignment[&'O'] as i32;
            let r: i32 = assignment[&'R'] as i32;
            let y: i32 = assignment[&'Y'] as i32;
            let send: i32 = s * 1000 + e * 100 + n * 10 + d;
            let more: i32 = m * 1000 + o * 100 + r * 10 + e;
            let money: i32 = m * 10000 + o * 1000 + n * 100 + e * 10 + y;
            return send + more == money;
        }
        true // no conflict
    }
    fn variables(&self) -> Vec<char> {
        self.0.clone()
    }
}

fn main() {
    // let mut args = env::args();
    // let n: i8 = match args.nth(1) {
    //     Some(n) => n.parse::<i8>().unwrap(),
    //     None => 8,
    // };
    // set up problem
    let vars: Vec<char> = vec!['S','E','N','D','M','O','R','Y'];
    let mut domains: FnvHashMap<char, Vec<i8>> = FnvHashMap::default();
    for v in vars.iter() {
        domains.insert(*v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    let mut csp: CSP<char, i8, SendMoneyConstraint> = CSP::new(vars.clone(), domains);

    // add constraints
    csp.add_constraint(SendMoneyConstraint(vars));
    // solve it
    let mut initial_guess: FnvHashMap<char, i8> = FnvHashMap::default();
    initial_guess.insert('M', 1);
    let solution = csp.backtracking_search(initial_guess);
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
