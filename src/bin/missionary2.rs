//! Solution to the Missionaries and Cannibals Problem
use pathfinding::prelude::bfs;
use std::fmt;

const MAX_NUM: i32 = 3;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MCState {
    wm: i32,
    wc: i32,
    em: i32,
    ec: i32,
    boat: bool,
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
        sucs.into_iter().filter(|x| x.is_legal()).collect()
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
    let start = MCState::new(MAX_NUM, MAX_NUM, true);
    static GOAL: MCState = MCState {
        wm: 0,
        wc: 0,
        em: MAX_NUM,
        ec: MAX_NUM,
        boat: false,
    };
    let result = bfs(&start, |p| p.successors(), |p| *p == GOAL);
    if result == None {
        println!("No solutions found.");
    } else {
        println!("{:?}", result);
        display_solution(result.unwrap());
    }
}
