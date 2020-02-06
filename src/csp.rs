use std::collections::HashMap;

trait Constraint {
    fn satisfied<V, D>(&self, assignment: HashMap<V, D>) -> bool;
}
