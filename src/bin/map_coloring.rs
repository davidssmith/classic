use std::collections::HashMap;
use std::hash::Hash;

extern crate classic;
use classic::csp::{CSP, Constraint};

/// Variables
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

/// Constraint: Proximity, touching can't have same color
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct MapColorConstraint(Place, Place);

impl Constraint<Place, Color> for MapColorConstraint {
    fn satisfied(&self, assignment: &HashMap<Place, Color>) -> bool {
        if !assignment.contains_key(&self.0) || !assignment.contains_key(&self.1) {
            return true;
        }
        return assignment[&self.0] != assignment[&self.1];
    }
    fn variables(&self) -> Vec<Place> {
        vec![self.0, self.1]
    }
}

fn main() {
    let variables: Vec<Place> = vec![
        Place::WA,
        Place::NT,
        Place::SA,
        Place::Q,
        Place::NSW,
        Place::V,
        Place::T,
    ];
    let mut domains: HashMap<Place, Vec<Color>> = HashMap::new();
    for &variable in variables.iter() {
        domains.insert(variable, vec![Color::Red, Color::Blue, Color::Green]);
    }
    //println!("variables: {:?}", variables);
    //println!("domains: {:?}", domains);
    let mut csp: CSP<Place, Color, MapColorConstraint> = CSP::new(variables, domains);
    csp.add_constraint(MapColorConstraint(Place::WA, Place::NT));
    csp.add_constraint(MapColorConstraint(Place::WA, Place::SA));
    csp.add_constraint(MapColorConstraint(Place::SA, Place::NT));
    csp.add_constraint(MapColorConstraint(Place::Q, Place::NT));
    csp.add_constraint(MapColorConstraint(Place::Q, Place::SA));
    csp.add_constraint(MapColorConstraint(Place::Q, Place::NSW));
    csp.add_constraint(MapColorConstraint(Place::NSW, Place::SA));
    csp.add_constraint(MapColorConstraint(Place::V, Place::SA));
    csp.add_constraint(MapColorConstraint(Place::V, Place::NSW));
    csp.add_constraint(MapColorConstraint(Place::V, Place::T));
    let mut initial_guess: HashMap<Place, Color> = HashMap::new();
    initial_guess.insert(Place::WA, Color::Red);
    let solution = csp.backtracking_search(initial_guess);
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
