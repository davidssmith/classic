//! connect 4 solver
use classic::minimax::find_best_move;
use std::{cmp, fmt};
use std::ops::{Index,IndexMut};
extern crate classic;
use classic::board::{Piece, Board, Move};
use text_io::read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum C4Piece {
    B,
    R,
    E,
}

impl Piece for C4Piece {
    fn opposite(&self) -> C4Piece {
        match self {
            C4Piece::B => C4Piece::R,
            C4Piece::R => C4Piece::B,
            _ => C4Piece::E,
        }
    }
}

impl fmt::Display for C4Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                C4Piece::B => "B",
                C4Piece::R => "R",
                C4Piece::E => " ",
            }
        )
    }
}


fn generate_segments(num_columns: u8, num_rows: u8, segment_length: u8) -> Vec<Vec<(u8,u8)>> {
    let segments: Vec<Vec<(u8,u8)>> = Vec::new();
    // generate the vertical segments
    for c in 0..num_columns {
        for r in 0..(NUM_ROWS - SEGMENT_LENGTH + 1) {
            let mut segment: Vec<(u8,u8)> = Vec::new();
            for t in 0..SEGMENT_LENGTH {
                segment.push((c, r + t));
            }
            segments.push(segment);
        }
    }

    // generate the horizontal segments
    for c in 0..(num_columns - SEGMENT_LENGTH + 1) {
        for r in 0..NUM_ROWS {
            let mut segment: Vec<(u8,u8)> = Vec::new();
            for t in 0..SEGMENT_LENGTH {
                segment.push((c + t, r));
            }
            segments.push(segment);
        }
    }

    // generate the bottom left to top right diagonal segments
    for c in 0..(num_columns - SEGMENT_LENGTH + 1) {
        for r in 0..NUM_ROWS - SEGMENT_LENGTH + 1 {
            let mut segment: Vec<(u8,u8)> = Vec::new();
            for t in 0..SEGMENT_LENGTH {
                segment.push((c + t, r + t));
            }
            segments.push(segment);
        }
    }

    // generate the top left to bottom right diagonal segments
    for c in 0..num_columns - SEGMENT_LENGTH + 1 {
        for r in (SEGMENT_LENGTH - 1)..NUM_ROWS {
            let mut segment: Vec<(u8,u8)> = Vec::new();
            for t in 0..SEGMENT_LENGTH {
                segment.push((c + t, r - t));
            }
            segments.push(segment);
        }
    }
    segments
}


#[derive(Clone,Debug,Eq,PartialEq)]
struct C4Column(Vec<C4Piece>);

impl C4Column {
    fn new(n: usize) -> C4Column {
        C4Column((0..n).map(|_| C4Piece::E).collect::<Vec<_>>())
    }

    fn full(&self) -> bool {
         self.0.len() == NUM_ROWS as usize
     }

    fn push(&self, item: C4Piece) {
        if self.full() {
            panic!("Trying to push piece to full column");
        }
        self.0.push(item);
    }
}

impl Index<usize> for C4Column {
    type Output = C4Piece;
    fn index(&self, index: usize) -> &Self::Output {
        if index as usize > self.0.len() - 1 {
            return &mut C4Piece::E;
        }
        &self.0[index]
    }
}

impl IndexMut<usize> for C4Column {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index as usize > self.0.len() - 1 {
            return &mut C4Piece::E;
        }
        &mut self.0[index]
    }
}

const NUM_ROWS: u8 = 6;
const NUM_COLS: u8 = 7;
const SEGMENT_LENGTH: u8 = 4;

#[derive(Clone,Debug,Eq,PartialEq)]
struct C4Board {
    position: Vec<C4Column>,
    turn: C4Piece,
    segments: Vec<Vec<(u8,u8)>>,
}

impl C4Board {
    fn new(position: Option<Vec<C4Column>>, turn: C4Piece) -> C4Board {
        // turn default: C4Piece::B
        let position = match position {
            None => (0..NUM_COLS).map(|_| C4Column::new(NUM_ROWS)).collect::<Vec<_>>(),
            Some(p) => p,
        };
        let segments: Vec<Vec<(u8,u8)>> = generate_segments(NUM_COLS, NUM_ROWS, SEGMENT_LENGTH);
        C4Board { position, turn, segments }

    }
    /// Returns the count of black & red pieces in a segment
    fn count_segment(&self, segment: Vec<(u8,u8)>) -> (u8,u8) {
        let mut black_count: u8 = 0;
        let mut red_count: u8 = 0;
        for (column, row) in segment {
            if self.position[column as usize][row as usize] == C4Piece::B {
                black_count += 1;
            } else if self.position[column as usize][row as usize] == C4Piece::R {
                red_count += 1;
            }
        }
        return (black_count, red_count)
    }
    fn evaluate_segment(self, segment: Vec<(u8,u8)>, player: C4Piece) -> f32 {
        let (black_count, red_count) = self.count_segment(segment);
        if red_count > 0 && black_count > 0 {
            return 0.0; // mixed segments are neutral
        }
        let count = cmp::max(red_count, black_count);
        let mut score: f32 = 0.0;
        if count == 2 {
            score = 1.0;
        } else if count == 3 {
            score = 100.0;
        } else if count == 4 {
            score = 1000000.0;
        }
        let mut color: C4Piece = C4Piece::B;
        if red_count > black_count {
            color = C4Piece::R;
        }
        if color != player {
            return -score;
        }
        score
    }
}

impl Board<C4Piece> for C4Board {
    fn turn(&self) -> C4Piece {
         self.turn
    }

    fn make_move(&self, location: Move) -> C4Board {
        let temp_position: Vec<C4Column> = self.position.clone();
        for c in 0..NUM_COLS {
            temp_position[c as usize] = self.position[c as usize].clone();
        }
        temp_position[location as usize].push(self.turn);
         C4Board { position: temp_position, turn: self.turn.opposite(), segments: self.segments.clone() }
     }

    fn legal_moves(&self) -> Vec<Move> {
        (0..NUM_COLS).filter(|&c| !self.position[c as usize].full()).collect::<Vec<_>>()
    }

    fn is_win(&self) -> bool {
        for segment in self.segments {
            let (black_count, red_count) = self.count_segment(segment);
            if black_count == 4 || red_count == 4 {
                return true;
            }
        }
         false
     }



    fn evaluate(&self, player: C4Piece) -> f32 {
        let mut total: f32 = 0.0;
        for segment in self.segments {
            total += self.evaluate_segment(segment, player);
        }
        return total;
    }
}


impl fmt::Display for C4Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for r in (0..NUM_ROWS).rev() {
            display.push_str("|");
            for c in 0..NUM_COLS {
                display.push_str(&format!("{}", self.position[c as usize][r as usize]));
                display.push_str("|");
            }
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}


fn get_player_move<B: Board<P>, P: Piece>(board: &B) -> Move {
    loop {
        let line: String = read!("{}\n");
        let player_move: Move = line.parse::<Move>().unwrap();
        if board.legal_moves().contains(&player_move) {
            return player_move;
        }
    }
}

fn main() {
    // main game loop

    let position = Some((0..9).map(|_| C4Piece::E).collect::<Vec<_>>());
    let mut board = C4Board::new(position, C4Piece::R);
    loop {
        let human_move = get_player_move(&board);
        board = board.make_move(human_move);
        println!("{}", board);
        if board.is_win() {
            println!("You win!");
            break;
        } else if board.is_draw() {
            println!("Draw!");
            break;
        }
        let computer_move = find_best_move(board.clone(), 8);
        println!("My move is {}", computer_move);
        board = board.make_move(computer_move);
        println!("{}", board);
        if board.is_win() {
            println!("I win!");
            break;
        } else if board.is_draw() {
            println!("Draw!");
            break;
        }
    }
}
