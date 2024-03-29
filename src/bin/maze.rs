//! maze solver
use ndarray::prelude::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Blocked,
    Start,
    Goal,
    Path,
}

type Grid = Array2<Cell>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct MazeLocation {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node<T>{
    state: T,
    parent: Option<Rc<Node<T>>>,
    cost: i32,
    heuristic: i32,
}

impl<T: PartialEq + Copy> Node<T> {
    fn to_path(&self) -> Vec<T> {
        let mut path: Vec<T> = vec![self.state];
        let mut node = self;
        while node.parent != None {
            node = &node.parent.as_ref().unwrap();
            path.push(node.state);
        }
        path.reverse();
        path
    }
}

impl<T: PartialEq + Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        let a = self.cost + self.heuristic;
        let b = other.cost + other.heuristic;
        a.partial_cmp(&b)
    }
}

impl<T: PartialEq + Eq> Ord for Node<T> {
    fn cmp(&self, other: &Node<T>) -> Ordering {
        let a = self.cost + self.heuristic;
        let b = other.cost + other.heuristic;
        a.cmp(&b)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Maze {
    rows: usize,
    cols: usize,
    sparseness: f32,
    start: MazeLocation,
    goal: MazeLocation,
    grid: Array2<Cell>,
}

impl Maze {
    fn new(rows: usize, cols: usize, sparseness: f32) -> Maze {
        let start = MazeLocation { row: 0, col: 0 };
        let goal = MazeLocation {
            row: rows as usize - 1,
            col: cols as usize - 1,
        };
        let grid = Grid::from_elem((rows, cols), Cell::Empty);
        let mut m = Maze {
            rows,
            cols,
            sparseness,
            start,
            goal,
            grid,
        };
        m.randomly_fill(sparseness);
        m
    }
    fn randomly_fill(&mut self, sparseness: f32) {
        let mut rng = rand::thread_rng();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if rng.gen::<f32>() < sparseness {
                    self.grid[[r, c]] = Cell::Blocked;
                }
            }
        }
        self.grid[[0, 0]] = Cell::Start;
        self.grid[[self.rows - 1, self.cols - 1]] = Cell::Goal;
    }
    fn goal(&self, ml: MazeLocation) -> bool {
        ml == self.goal
    }
    fn successors(&self, m: MazeLocation) -> Vec<MazeLocation> {
        let r = m.row;
        let c = m.col;
        let mut locations: Vec<MazeLocation> = Vec::with_capacity(4);
        if r + 1 < self.rows as usize && self.grid[[r + 1, c]] != Cell::Blocked {
            locations.push(MazeLocation { row: r + 1, col: c })
        }
        if m.col + 1 < self.cols as usize && self.grid[[r, c + 1]] != Cell::Blocked {
            locations.push(MazeLocation { row: r, col: c + 1 })
        }
        if m.row != 0 && self.grid[[r - 1, c]] != Cell::Blocked {
            locations.push(MazeLocation { row: r - 1, col: c })
        }
        if m.col != 0 && self.grid[[r, c - 1]] != Cell::Blocked {
            locations.push(MazeLocation { row: r, col: c - 1 })
        }
        locations
    }
    /// Breadth-first search
    // fn bfs(&self, initial: MazeLocation) -> Option<Node<MazeLocation>> {
    //     let mut frontier: VecDeque<Node<MazeLocation>> = VecDeque::new();
    //     frontier.push_back(Node {
    //         state: initial,
    //         parent: None,
    //         cost: 0,
    //         heuristic: 0,
    //     });
    //     let mut seen: HashSet<MazeLocation> = HashSet::new(); // TODO: replace with Array2?
    //     while !frontier.is_empty() {
    //         let cur_node = Rc::new(frontier.pop_front().unwrap());
    //         let cur_state = cur_node.state;
    //         if self.goal(cur_state) {
    //             return Some(Rc::try_unwrap(cur_node).unwrap());
    //         }
    //         for child in self.successors(cur_state) {
    //             if seen.contains(&child) {
    //                 continue;
    //             }
    //             seen.insert(child);
    //             frontier.push_back(Node {
    //                 state: child,
    //                 parent: Some(cur_node.clone()),
    //                 cost: 0,
    //                 heuristic: 0,
    //             })
    //         }
    //     }
    //     None
    // }
    // /// Depth-first search
    // fn dfs(&self, initial: MazeLocation) -> Option<Node<MazeLocation>> {
    //     let mut frontier: Vec<Node<MazeLocation>> = Vec::new();
    //     frontier.push(Node {
    //         state: initial,
    //         parent: None,
    //         cost: 0,
    //         heuristic: 0,
    //     });
    //     let mut seen: HashSet<MazeLocation> = HashSet::new(); // TODO: replace with Array2?
    //     while !frontier.is_empty() {
    //         let cur_node = Rc::new(frontier.pop().unwrap());
    //         let cur_state = cur_node.state;
    //         if self.goal(cur_state) {
    //             return Some(Rc::try_unwrap(cur_node).unwrap());
    //         }
    //         for child in self.successors(cur_state) {
    //             if seen.contains(&child) {
    //                 continue;
    //             }
    //             seen.insert(child);
    //             frontier.push(Node {
    //                 state: child,
    //                 parent: Some(cur_node.clone()),
    //                 cost: 0,
    //                 heuristic: 0,
    //             })
    //         }
    //     }
    //     None
    // }
    /// Depth-first search
    fn astar(&self, initial: MazeLocation) -> Option<Node<MazeLocation>> {
        let mut frontier: BinaryHeap<Node<MazeLocation>> = BinaryHeap::new();
        frontier.push(Node {
            state: initial,
            parent: None,
            cost: 0,
            heuristic: 0,
        });
        let mut seen: HashSet<MazeLocation> = HashSet::new(); // TODO: replace with Array2?
        while !frontier.is_empty() {
            let cur_node = Rc::new(frontier.pop().unwrap());
            let cur_state = cur_node.state;
            if self.goal(cur_state) {
                return Some(Rc::try_unwrap(cur_node).unwrap());
            }
            for child in self.successors(cur_state) {
                if seen.contains(&child) {
                    continue;
                }
                seen.insert(child);
                frontier.push(Node {
                    state: child,
                    parent: Some(cur_node.clone()),
                    cost: cur_node.cost - 1,
                    heuristic: 0,
                });
            }
        }
        None
    }
    fn mark(&mut self, path: &Vec<MazeLocation>) {
        for p in path {
            self.grid[[p.row, p.col]] = Cell::Path;
        }
        self.grid[[self.start.row, self.start.col]] = Cell::Start;
        self.grid[[self.goal.row, self.goal.col]] = Cell::Goal;
    }
    fn clear(&mut self, path: &Vec<MazeLocation>) {
        for p in path {
            self.grid[[p.row, p.col]] = Cell::Empty;
        }
        self.grid[[self.start.row, self.start.col]] = Cell::Start;
        self.grid[[self.goal.row, self.goal.col]] = Cell::Goal;
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, g) in self.grid.iter().enumerate() {
            s.push_str(match g {
                Cell::Empty => " ",
                Cell::Blocked => "0",
                Cell::Start => "S",
                Cell::Goal => "G",
                Cell::Path => ".",
            });
            if i % self.rows == 0 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let mut m = Maze::new(20, 20, 0.2);
    println!("{}", m);
    let sol1 = m.astar(m.start);
    if sol1 == None {
        println!("No solution found using depth-first search.");
    } else {
        let path1 = sol1.unwrap().to_path();
        m.mark(&path1);
        println!("{}", m);
        m.clear(&path1);
    }
}
