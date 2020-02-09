//! graph framework
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Graph edge object
pub struct Edge {
    pub u: i32, // "from" vertex
    pub v: i32, // "to" vertex
}

impl Edge {
    fn new(u: i32, v: i32) -> Edge {
        Edge { u, v }
    }
    fn reversed(&self) -> Edge {
        Edge {
            u: self.v,
            v: self.u,
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.u, self.v)
    }
}

impl From<(i32, i32)> for Edge {
    fn from(tuple: (i32, i32)) -> Edge {
        Edge {
            u: tuple.0,
            v: tuple.1,
        }
    }
}

/// Graph object
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Graph<V> {
    vertices: Vec<V>,
    edges: Vec<Vec<Edge>>,
}

impl<V> Graph {
    fn vertex_count(&self) -> i32 {
        self.vertices.len()
    }
    fn edge_count(&self) -> i32 {
        self.edges.iter().map(|edge| edge.len()).sum()
    }
    fn add_vertex(&mut self, vertex: V) -> i32 {
        self.vertices.push(vertex);
        self.edges.push(Vec::new());
        self.vertex_count() - 1
    }
}
