//! connect 4 solver
use std::cmp;
extern crate board;
use classic::board::{Piece, Board, Move};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum C4Piece {
    B, // "X"
    R, // "O"
    E, // " "
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


fn generate_segments(num_columns: i32, num_rows: i32, segment_length: i32) -> Vec<Vec<(i32,i32)>> {
    let segments: Vec<Vec<(i32,i32)>> = Vec::new();
    // generate the vertical segments
    for c in 0..num_columns {
        for r in 0..num_rows - segment_length + 1 {
            let mut segment: Vec<(i32,i32)> = Vec::new();
            for t in 0..segment_length {
                segment.push((c, r + t));
            }
            segments.push(segment);
        }
    }

    // generate the horizontal segments
    for c in 0..num_columns - segment_length + 1 {
        for r in 0..num_rows {
            let mut segment: Vec<(i32,i32)> = Vec::new();
            for t in 0..segment_length {
                segment.push((c + t, r));
            }
            segments.push(segment);
        }
    }

    // generate the bottom left to top right diagonal segments
    for c in 0..num_columns - segment_length + 1 {
        for r in 0..num_rows - segment_length + 1 {
            let mut segment: Vec<(i32,i32)> = Vec::new();
            for t in 0..segment_length {
                segment.push((c + t, r + t));
            }
            segments.push(segment);
        }
    }

    // generate the top left to bottom right diagonal segments
    for c in 0..num_columns - segment_length + 1 {
        for r in 0..segment_length - 1, num_rows {
            let mut segment: Vec<(i32,i32)> = Vec::new();
            for t in 0..segment_length {
                segment.push((c + t, r - t));
            }
            segments.push(segment);
        }
    }
    segments
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct C4Board {
    num_rows: i32, // = 6
    num_cols: i32, // = 7
    segment_length: i32, // = 4
    segments: Vec<Vec<(i32,i32)>>,
    // = generate_segments(num_cols, num_rows, segment_length)
}

impl C4Board {
fn new(position: Option<Vec<C4Column>>, turn: C4Piece = C4Piece::B) {
    if position is None:
        self.position: Vec<C4Column> = [C4Column::new() for _ in 0..C4Board.num_cols)]
    else:
        self.position = position
    self.turn: C4Piece = turn
}
/// Returns the count of black & red pieces in a segment
fn count_segment(&self, segment: Vec<(i32,i32)>) -> (i32,i32) {
    let mut black_count: i32 = 0;
    let mut red_count: i32 = 0;
    for (column, row) in segment {
        if self.position[column][row] == C4Piece::B {
            black_count += 1;
        } else if self.position[column][row] == C4Piece::R {
            red_count += 1;
        }
    }
    return (black_count, red_count)
}
fn evaluate_segment(self, segment: Vec<(i32,i32)>, player: Piece) -> f32 {
    let (black_count, red_count) = self.count_segment(segment);
    if red_count > 0 && black_count > 0 {
        return 0; // mixed segments are neutral
    }
    let count = cmp::max(red_count, black_count);
    let mut score: f32 = 0.0;
    if count == 2 {
        score = 1;
    } else if count == 3 {
        score = 100;
    } else if count == 4 {
        score = 1000000;
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
    temp_position: Vec[C4Board.Column] = self.position.clone();
    for c in 0..C4Board.num_cols):
        temp_position[c] = self.position[c].copy();
    temp_position[location].push(self.turn);
     C4Board(temp_position, self.turn.opposite)
 }

fn legal_moves(&self) -> Vec<Move> {
    return [Move(c) for c in 0..C4Board.num_cols) if not self.position[c].full]
}

fn is_win(&self) -> bool {
    for segment in C4Board.segments {
        let (black_count, red_count) = self.count_segment(segment);
        if black_count == 4 || red_count == 4 {
            return true;
        }
    }
     false
 }



fn evaluate(&self, player: Piece) -> f32 {
    let mut total: f32 = 0.0;
    for segment in C4Board.segments {
        total += self.evaluate_segment(segment, player);
    }
    return total;
}
}


def __repr__(&self) -> str:
    display: str = ""
    for r in reversed(0..C4Board.num_rows)):
        display += "|"
        for c in 0..C4Board.num_cols):
            display += f"{self.position[c][r]}" + "|"
        display += "\n"
    return display

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct C4Column(Vec<C4Piece>);

impl C4Column {
        fn new() -> C4Column {
            C4Column(Vec::new())
        }

        fn full(&self) -> bool {
             self.0.len() == C4Board.num_rows
         }

        fn push(&self, item: C4Piece) {
            if self.full() {
                panic!("Trying to push piece to full column");
            }
            self.0.push(item);
        }

        fn __getitem__(&self, index: i32) -> C4Piece {
            if index > self.0.len() - 1 {
                return C4Piece::E
            }
            self.0[index]
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
    let mut board = C4Board {
        position: (0..9).map(|_| C4Piece::E).collect(),
        turn: C4Piece::X,
    };
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
