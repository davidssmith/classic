use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub trait Constraint<V: Eq + Hash, D> {
    fn satisfied(&self, assignment: HashMap<V,D>) -> bool;
    fn variables(&self) -> Vec<V>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CSP<V,D,C> where V: Clone + Copy + Eq + Hash, C: Constraint<V,D> {
    variables: Vec<V>,
    domains: HashMap<V,Vec<D>>,
    constraints: HashMap<V,Vec<C>>,
}

impl<V: Clone + Copy + Eq + Hash, D: Clone, C: Constraint<V,D>> CSP<V,D,C> {
    fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> CSP<V,D,C> {
        let mut constraints: HashMap<V,Vec<C>> = HashMap::new();
        for variable in &variables {
            constraints.insert(*variable, Vec::new());
            if !domains.contains_key(&variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        CSP { variables, domains, constraints }
    }
    fn add_constraint(&mut self, constraint: C) {
        for variable in &constraint.variables() {
            if !self.variables.contains(&variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                self.constraints.get_mut(&variable).unwrap().push(constraint)
            }
        }
    }
    fn consistent(&self, variable: V, assignment: &HashMap<V,D>) -> bool {
        self.constraints[&variable].iter().any(|&c| !c.satisfied(*assignment))
    }
    fn backtracking_search(&self, assignment: HashMap<V,D>) -> Option<HashMap<V,D>> {
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }
        let unassigned: Vec<V> = self.variables.clone().into_iter()
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

/// Constraint: Proximity, touching can't have same color
#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct MapColorConstraint(Place,Place);


impl Constraint<Place,Color> for MapColorConstraint {
    fn satisfied(&self, assignment: HashMap<Place,Color>) -> bool {
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
    let variables: Vec<Place> = vec![Place::WA, Place::NT, Place::SA, Place::Q, Place::NSW, Place::V, Place::T];
    let mut domains: HashMap<Place, Vec<Color>> = HashMap::new();
    for &variable in variables.iter() {
        domains.insert(variable, vec![Color::Red, Color::Blue, Color::Green]);
    }
    println!("variables: {:?}", variables);
    println!("domains: {:?}", domains);
    let mut csp: CSP<Place,Color,MapColorConstraint> = CSP::new(variables, domains);
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
    let mut initial_guess: HashMap<Place,Color> = HashMap::new();
    initial_guess.insert(Place::WA, Color::Blue);
    let solution = csp.backtracking_search(initial_guess);
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
