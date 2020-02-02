//! maze solver
use ndarray::prelude::*;
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Blocked,
    Start,
    Goal,
    Path,
}

type Grid = Array2<Cell>;

#[derive(Clone, Copy, PartialEq, Debug)]
struct MazeLocation {
    row: usize,
    col: usize,
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
    fn goal_test(&self, ml: MazeLocation) -> bool {
        ml == self.goal
    }
    fn successors(&self, m: MazeLocation) -> Vec<MazeLocation> {
        let r = m.row;
        let c = m.col;
        let mut locations: Vec<MazeLocation> = Vec::with_capacity(4);
        if r + 1 < self.rows as usize && self.grid[[r + 1, c]] != Cell::Blocked {
            locations.push(MazeLocation { row: r + 1, col: c})
        }
        if m.col + 1 < self.cols as usize && self.grid[[r, c + 1]] != Cell::Blocked {
            locations.push(MazeLocation { row: r, col: c + 1})
        }
        if m.row != 0 && self.grid[[r - 1, c]] != Cell::Blocked {
            locations.push(MazeLocation { row: r - 1, col: c })
        }
        if m.col != 0 && self.grid[[r, c - 1]] != Cell::Blocked {
            locations.push(MazeLocation { row: r, col: c - 1 })
        }
        locations
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, g) in self.grid.iter().enumerate() {
            s.push_str(match g {
                Cell::Empty => " ",
                Cell::Blocked => "O",
                Cell::Start => "S",
                Cell::Goal => "G",
                Cell::Path => "*",
            });
            if i % self.rows == 0 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let m = Maze::new(10, 10, 0.2);
    println!("{}", m);
}
