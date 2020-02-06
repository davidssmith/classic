//! n-queens solver
use std::collections::HashMap;
use crate::csp::{CSP, Constraint};

struct QueensConstrait {
    cols: Vec<i32>,
}

impl Constraint for QueensConstraint {
    fn satisfied<V, D>(&self, assignment: HashMap<V, D>) -> bool {
        // q1c = queen 1 column, q1r = queen 1 row
        for (q1c, q1r) in assignment.items() {
            for q2c in q1c + 1..self.cols.len() + 1 {
                if q2c in assignment {
                    let q2r = assignment(q2c);
                    if q1r == q2r {
                    return false;
                } else if abs(q1r - q2r) == abs(q1c - q2c) {
                    return false;
                }
            }
        }

    }

}

fn main() {
    let cols: [i32; 8] = [1, 2, 3, 4, 5, 6, 7 ,8];
    let rows: HashMap<i32, Vec<i32>> = HashMap::new();
    for c in cols.iter() {
        rows[c] = [1, 2, 3, 4, 5, 6, 7, 8];
    }
    let csp: CSP<i32,i32> = CSP(columns, rows);
}
