//! Solution to the Missionaries and Cannibals Problem
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::rc::Rc;

const MAX_NUM: i32 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct MCState {
    wm: i32,
    wc: i32,
    em: i32,
    ec: i32,
    boat: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node<T> {
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

impl MCState {
    fn new(wm: i32, wc: i32, boat: bool) -> MCState {
        MCState {
            wm,
            wc,
            em: MAX_NUM - wm,
            ec: MAX_NUM - wc,
            boat,
        }
    }
    fn is_legal(&self) -> bool {
        if self.wm < self.wc && self.wm > 0 {
            return false;
        }
        if self.em < self.ec && self.em > 0 {
            return false;
        }
        return true;
    }
    fn successors(&self) -> Vec<MCState> {
        let mut sucs: Vec<MCState> = Vec::new();
        if self.boat {
            if self.wm > 1 {
                sucs.push(MCState::new(self.wm - 2, self.wc, !self.boat));
            }
            if self.wm > 0 {
                sucs.push(MCState::new(self.wm - 1, self.wc, !self.boat));
            }
            if self.wc > 1 {
                sucs.push(MCState::new(self.wm, self.wc - 2, !self.boat));
            }
            if self.wc > 0 {
                sucs.push(MCState::new(self.wm, self.wc - 1, !self.boat));
            }
            if self.wm > 0 && self.wc > 0 {
                sucs.push(MCState::new(self.wm - 1, self.wc - 1, !self.boat));
            }
        } else {
            if self.em > 1 {
                sucs.push(MCState::new(self.wm + 2, self.wc, !self.boat));
            }
            if self.em > 0 {
                sucs.push(MCState::new(self.wm + 1, self.wc, !self.boat));
            }
            if self.ec > 1 {
                sucs.push(MCState::new(self.wm, self.wc + 2, !self.boat));
            }
            if self.ec > 0 {
                sucs.push(MCState::new(self.wm, self.wc + 1, !self.boat));
            }
            if self.em > 0 && self.ec > 0 {
                sucs.push(MCState::new(self.wm + 1, self.wc + 1, !self.boat));
            }
        }
        let sucs_filtered: Vec<MCState> = sucs.into_iter().filter(|&x| x.is_legal()).collect();
        sucs_filtered
    }
    fn goal(&self) -> bool {
        self.is_legal() && self.em == MAX_NUM && self.ec == MAX_NUM
    }
    /// Breadth-first search
    fn bfs(&mut self) -> Option<Node<MCState>> {
        let initial = self.clone();
        let mut frontier: VecDeque<Node<MCState>> = VecDeque::new();
        frontier.push_back(Node {
            state: initial,
            parent: None,
            cost: 0,
            heuristic: 0,
        });
        let mut seen: HashSet<MCState> = HashSet::new(); // TODO: replace with Array2?
        while !frontier.is_empty() {
            let cur_node = Rc::new(frontier.pop_front().unwrap());
            let cur_state = cur_node.state;
            if cur_state.goal() {
                return Some(Rc::try_unwrap(cur_node).unwrap());
            }
            for child in cur_state.successors() {
                if seen.contains(&child) {
                    continue;
                }
                seen.insert(child);
                frontier.push_back(Node {
                    state: child,
                    parent: Some(cur_node.clone()),
                    cost: 0,
                    heuristic: 0,
                });
            }
        }
        None
    }
}

impl fmt::Display for MCState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "On the west bank there are {} missionaries and {} cannibals.\n\
             On the east bank there are {} missionaries and {} cannibals.\n\
             The boat is on the {} bank.",
            self.wm,
            self.wc,
            self.em,
            self.ec,
            if self.boat { "west" } else { "east" }
        )
    }
}

fn display_solution(path: Vec<MCState>) {
    if path.len() == 0 {
        return;
    }
    let mut old_state = path[0];
    println!("{}", old_state);
    for current_state in path[1..].iter() {
        if current_state.boat {
            println!(
                "{} missionaries and {} cannibals east -> west.",
                old_state.em - current_state.em,
                old_state.ec - current_state.ec
            );
        } else {
            println!(
                "{} missionaries and {} cannibals west -> east.",
                old_state.wm - current_state.wm,
                old_state.wc - current_state.wc
            );
        }
        println!("{}", current_state);
        old_state = *current_state;
    }
}

fn main() {
    let mut start = MCState::new(MAX_NUM, MAX_NUM, true);
    let solution = start.bfs();
    if solution == None {
        println!("No solutions found.");
    } else {
        display_solution(solution.unwrap().to_path());
    }
}
