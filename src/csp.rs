use std::collections::HashMap;
use std::hash::Hash;

pub trait Constraint<V: Hash + Eq, D> {
    fn satisfied(&self, assignment: HashMap<V, D>) -> bool;
}

//V = TypeVar('V') # variable type
//D = TypeVar('D') # domain type

// A constraint satisfaction problem consists of variables of type V
// that have ranges of values known as domains of type D and constraints
// that determine whether a particular variable's domain selection is valid
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CSP<V: Hash + Eq + PartialEq, D: Hash, C: Constraint<V,D>> {
    variables: Vec<V>,
    domains: HashMap<V, Vec<D>>,
    constraints: HashMap<V, Vec<C>>,
}


impl<V: Hash + Eq + PartialEq, D> CSP<V,D> {
    fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> CSP<V,D> {
        let mut constraints: HashMap<V,Vec<Constraint<V,D>>> = HashMap::new();
        for variable in variables {
            constraints.insert(variable, Vec::new());
            if !self.domains.contains_key(variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        CSP { variables, domains, constraints }
    }
    fn add_constraint(&self, constraint: Constraint<V,D>) {
        for variable in constraint.variables.iter() {
            if !self.variables.contains_kep(variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                self.constraints[variable].append(constraint)
            }
        }
    }

    // Check if the value assignment is consistent by checking all constraints
    // for the given variable against it
    fn consistent(&self, variable: V, assignment: HashMap<V,D>) -> bool {
        for constraint in self.constraints[variable] {
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
