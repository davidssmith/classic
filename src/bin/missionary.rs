//! Solution to the Missionaries and Cannibals Problem
use std::fmt;

const MAX_NUM: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct MCState {
    wm: usize,
    wc: usize,
    em: usize,
    ec: usize,
    boat: bool,
}

impl MCState {
    fn new(wm: usize, wc: usize, boat: bool) -> MCState {
        MCState {
            wm,
            wc,
            em: MAX_NUM - wc,
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
        let sucs: Vec<MCState> = Vec::new();
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
        sucs.iter().filter(|x| x.is_legal()).collect()
    }
}

impl fmt::Display for MCState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "On the west bank there are {} missionaries and {} cannibals.",
            self.wm, self.wc
        );
        write!(
            f,
            "On the east bank there are {} missionaries and {} cannibals.",
            self.em, self.ec
        );
        write!(
            f,
            "The boat is on the {} bank.",
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

fn main() {
    let start = MCState::new(MAX_NUM, MAX_NUM, true);
    let solution = bfs(start, MCState.goal_test, MCState.successors);
    if solution == None {
        println!("No solutions found.");
    } else {
        let path = node_to_path(solution);
        display_solution(path);
    }
}
