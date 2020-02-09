use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub trait Constraint<V: Eq + Hash, D> {
    fn satisfied(&self, assignment: HashMap<V,D>) -> bool;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CSP<V: Eq + Hash, D, C: Constraint<V,D>> {
    variables: Vec<V>,
    domains: HashMap<V,Vec<D>>,
    constraints: HashMap<V,Vec<C>>,
}

impl<V: Eq + Hash, D, C: Constraint<V,D>> CSP<V,D,C> {
    fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> CSP<V,D,C> {
        let mut constraints: HashMap<V,Vec<C>> = HashMap::new();
        for variable in variables {
            constraints.insert(variable, Vec::new());
            if !domains.contains_key(&variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        CSP { variables, domains, constraints }
    }
    fn add_constraint(&self, constraint: C) {
        for variable in constraint.variables.iter() {
            if !self.variables.contains(&variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                self.constraints[&variable].append(&mut constraint)
            }
        }
    }
    fn consistent(&self, variable: V, assignment: HashMap<V,D>) -> bool {
        self.constraints[&variable].iter().any(|&c| !c.satisfied(assignment))
    }
    fn backtracking_search(&self, assignment: HashMap<V,D>) -> Option<HashMap<V,D>> {
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }
        let unassigned: Vec<V> = self.variables.iter()
            .filter(|&v| !assignment.contains_key(v))
            .collect();
        let first: V = unassigned[0];
        for value in self.domains[&first] {
            let local_assignment = assignment.clone();
            local_assignment[first] = value;
            if self.consistent(first, local_assignment) {
                let result = self.backtracking_search(local_assignment);
                if result != None { // TODO: make more elegant
                    return result.unwrap();
                }
            }
        }
        None
    }
}

/// Variables
#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
enum Place {
    WA,
    NT,
    SA,
    Q,
    NSW,
    V,
    T,
}

/// Domains
#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

// /// Constraints: Proximity, touching can't have same color
// #[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
// struct MapColoringConstraint(Place,Place);
//
//
// impl MapColoringConstraint {
//     fn satisfied(&self, assignment: HashMap<Place,Color>) -> bool {
//         if !assignment.contains_key(&self.0) || !assignment.contains_key(&self.1) {
//             return true;
//         }
//         // check the color assigned to place1 is not the same as the
//         // color assigned to place2
//         return assignment[&self.0] != assignment[&self.1];
//     }
// }
//
fn main() {
    let variables: Vec<Place> = vec![Place::WA, Place::NT, Place::SA, Place::Q, Place::NSW, Place::V, Place::T];
    let mut domains: HashMap<Place, Vec<Color>> = HashMap::new();
    for variable in variables.iter() {
        domains[variable] = vec![Color::Red, Color::Blue, Color::Green];
    }
    println!("variables: {:?}", variables);
    println!("domains: {:?}", domains);
//     let mut csp: CSP<Place,Color,MapColoringConstraint<Place,Color>> = CSP::new(variables, domains);
//     csp.add_constraint(MapColoringConstraint(Place::WA, Place::NT));
//     csp.add_constraint(MapColoringConstraint(Place::WA, Place::SA));
//     csp.add_constraint(MapColoringConstraint(Place::SA, Place::NT));
//     csp.add_constraint(MapColoringConstraint(Place::Q, Place::NT));
//     csp.add_constraint(MapColoringConstraint(Place::Q, Place::SA));
//     csp.add_constraint(MapColoringConstraint(Place::Q, Place::NSW));
//     csp.add_constraint(MapColoringConstraint(Place::NSW, Place::SA));
//     csp.add_constraint(MapColoringConstraint(Place::V, Place::SA));
//     csp.add_constraint(MapColoringConstraint(Place::V, Place::NSW));
//     csp.add_constraint(MapColoringConstraint(Place::V, Place::T));
//     let solution = csp.backtracking_search();
//     if solution == None {
//         println!("No solution found!");
//     } else {
//         println!("{:?}", solution);
//     }
}
