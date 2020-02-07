use std::collections::HashMap;
use csp::{Constraint, CSP};

struct MapColoringConstraint<'a> {
    place1: &'a str,
    place2: &'a str,
}

impl<'a> MapColoringConstraint {
    fn new(place1: &'a str, place2: &'a str) -> MapColoringConstraint<'a> {
        MapColoringConstraint { place1, place2 }
    }
    fn satisfied(&self, assignment: HashMap<&str, &str>) -> bool {
        // If either place is not in the assignment then it is not
        // yet possible for their colors to be conflicting
        if self.place1 not in assignment or self.place2 not in assignment {
            return true;
        }
        // check the color assigned to place1 is not the same as the
        // color assigned to place2
        return assignment[self.place1] != assignment[self.place2];
    }
}

fn main() {
    variables: Vec<&str> = vec!["Western Australia", "Northern Territory", "South Australia",
                            "Queensland", "New South Wales", "Victoria", "Tasmania"];
    let mut domains: HashMap<&str, Vec<&str>> = HashMap::new();
    for variable in variables.iter() {
        domains[variable] = vec!["red", "green", "blue"];
    }
    let mut csp: CSP<&str,&str> = CSP::new(variables, domains);
    csp.add_constraint(MapColoringConstraint::new("Western Australia", "Northern Territory"));
    csp.add_constraint(MapColoringConstraint::new("Western Australia", "South Australia"));
    csp.add_constraint(MapColoringConstraint::new("South Australia", "Northern Territory"));
    csp.add_constraint(MapColoringConstraint::new("Queensland", "Northern Territory"));
    csp.add_constraint(MapColoringConstraint::new("Queensland", "South Australia"));
    csp.add_constraint(MapColoringConstraint::new("Queensland", "New South Wales"));
    csp.add_constraint(MapColoringConstraint::new("New South Wales", "South Australia"));
    csp.add_constraint(MapColoringConstraint::new("Victoria", "South Australia"));
    csp.add_constraint(MapColoringConstraint::new("Victoria", "New South Wales"));
    csp.add_constraint(MapColoringConstraint::new("Victoria", "Tasmania"));
    let solution = csp.backtracking_search();
    if solution == None {
        println!("No solution found!");
    } else {
        println!("{:?}", solution);
    }
}
