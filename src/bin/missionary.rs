//! Solution to the Missionaries and Cannibals Problem
use std::fmt;

const MAX_NUM: usize = 3;

struct MCState {
    wm: usize,
    wc: usize,
    em: usize,
    ec: usize,
    boat: bool,
}


impl MCState {
    fn new(wm: usize, wc: usize, boat: bool) -> MCState {
        MCState { wm, wc, em: MAX_NUM - wc, ec: MAX_NUM - wc, boat }
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
        let sucs : Vec<MCState> = Vec::new();
    }
}

impl fmt::Display for MCState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "On the west bank there are {} missionaries and {} cannibals.", self.wm, self.wc);
        write!(f, "On the east bank there are {} missionaries and {} cannibals.", self.em, self.ec);
        write!(f, "The boat is on the {} bank.", if self.boat { "west" } else { "east" })
    }
}
