//! tic-tac-toe solver
use std::fmt;
use rand::prelude::*;

extern crate classic;
use classic::board::{Board, Move, Piece};
use classic::minimax::find_best_move;
use text_io::read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OwarePiece {
    P1,
    P2,
}

impl Piece for OwarePiece {
    fn opposite(&self) -> OwarePiece {
        match self {
            OwarePiece::P1 => OwarePiece::P2,
            OwarePiece::P2 => OwarePiece::P1,
        }
    }
}

impl fmt::Display for OwarePiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OwarePiece::P1 => "1",
                OwarePiece::P2 => "2",
            }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OwareBoard {
    position: [u8; 12],
    turn: OwarePiece,
    score1: u8,
    score2: u8,
}

impl Default for OwareBoard {
    fn default() -> OwareBoard {
        OwareBoard {
            position: [4u8; 12],
            turn: OwarePiece::P1,
            score1: 0,
            score2: 0,
        }
    }

}

impl Board<OwarePiece> for OwareBoard {
    fn turn(&self) -> OwarePiece {
        self.turn
    }
    fn make_move(&self, loc: Move) -> OwareBoard {
        let n_to_sow = self.position[loc as usize] as usize; // num seeds to sow
        let mut b = OwareBoard {
            position: self.position.clone(),
            turn: self.turn.opposite(),
            score1: self.score1,
            score2: self.score2,
        };
        b.position[loc as usize] = 0;
        for i in 0..n_to_sow { // sow
            let h = (i + loc as usize + 1) % 12;
            b.position[h] += 1;
        }
        for i in (0..n_to_sow).rev() { // score
            let h = (i + loc as usize + 1) % 12;
            let num_before = self.position[h];
            if num_before == 0 || num_before > 2 {
                break;
            }
            if self.turn == OwarePiece::P1 && h >= 6 {
                b.score1 += b.position[h];
                b.position[h] = 0;
            } else if self.turn == OwarePiece::P2 && h < 6 {
                b.score2 += b.position[h];
                b.position[h] = 0;
            } else {
                break;
            }
        }
        b
    }
    fn legal_moves(&self) -> Vec<Move> {
        let mut moves = match self.turn {
            OwarePiece::P1 => (0..6).filter(|&x| self.position[x] > 0)
                 .map(|x| x as Move).collect::<Vec<_>>(),
            OwarePiece::P2 => (6..12).filter(|&x| self.position[x] > 0)
                 .map(|x| x as Move).collect::<Vec<_>>(),
        };
        let mut rng = rand::thread_rng(); //: StdRng = SeedableRng::seed_from_u64(0xc0ffee); //rand::thread_rng();
        moves.shuffle(&mut rng);  // randomize the order of move searching (HashSet faster?)
        moves
            // TODO: make forced move if opponent has no seeds
    }
    fn is_win(&self) -> bool {
        // three row, three column, and then two diagonal checks
        self.score1 > 24 || self.score2 > 24 || self.legal_moves().is_empty()
    }
    fn evaluate(&self, player: OwarePiece) -> f32 {
        let mut score_diff = self.score1 as f32 - self.score2 as f32;
        if player == OwarePiece::P2 {
            score_diff = -score_diff;
        }
        if self.turn == player {
            return -score_diff;
        } else {
            return score_diff;
        }
    }
}

fn num_to_dots(n: u8) -> String {
    match n {
        0 => "  ".to_owned(),
        1 => " ⠂".to_owned(),
        2 => " ⠒".to_owned(),
        3 => " ⠲".to_owned(),
        4 => " ⠶".to_owned(),
        5 => " ⁙".to_owned(),
        6 => " ⠿".to_owned(),
        _ => format!("{:2}", n),
    }
}

impl fmt::Display for OwareBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:2} | {} {} {} {} {} {}\n     {} {} {} {} {} {} | {:2}\n      1  2  3  4  5  6",
            self.score2,
            num_to_dots(self.position[11]),
            num_to_dots(self.position[10]),
            num_to_dots(self.position[9]),
            num_to_dots(self.position[8]),
            num_to_dots(self.position[7]),
            num_to_dots(self.position[6]),
            num_to_dots(self.position[0]),
            num_to_dots(self.position[1]),
            num_to_dots(self.position[2]),
            num_to_dots(self.position[3]),
            num_to_dots(self.position[4]),
            num_to_dots(self.position[5]),
            self.score1
        )
    }
}

fn get_player_move<B: Board<P>, P: Piece>(board: &B) -> Move {
    loop {
        let line: String = read!("{}\n");
        let player_move: Move = line.parse::<Move>().unwrap() - 1;
        if board.legal_moves().contains(&player_move) {
            return player_move;
        }
    }
}

fn main() {
    let ngames = 100;
    let search_depth = 7;
    let mut wins1 = 0;
    let mut wins2 = 0;
    let mut draws = 0;
    let first_move = 2;
    for g in 0..ngames {
        print!("game {}: ", g);
        let mut board = OwareBoard::default();
        let mut m = 0;
        loop {
            m += 1;
            //println!("{}.", m);
            let p1 = find_best_move(board.clone(), search_depth);
            board = board.make_move(p1);
            //println!("{}", board);
            if board.is_win() {
                wins1 += 1;
                println!("1");
                break;
            } else if board.is_draw() {
                draws += 1;
                println!("draw");
                break;
            }
            let p2 = find_best_move(board.clone(), search_depth);
            board = board.make_move(p2);
            //println!("{}", board);
            if board.is_win() {
                wins2 += 1;
                println!("2");
                break;
            } else if board.is_draw() {
                draws += 1;
                println!("draw");
                break;
            }
        }
    }
    let ngames = ngames as f32;
    println!("wins1: {}  wins2: {}  draws: {}", wins1 as f32 / ngames, wins2 as f32 / ngames,
        draws as f32 / ngames);
}

fn main2() {
    // main game loop
    let mut board = OwareBoard::default();
    let search_depth = 2;
    println!("Welcome to Oware! You are Player 1. I am searching to depth {}.\n\
              Your houses are labelled 1-6. Enter a number to move. Good luck!", search_depth);
    let mut turn = 1;
    println!("{}", board);
    loop {
        println!("Turn {}", turn);
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
        let computer_move = find_best_move(board.clone(), search_depth);
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
        turn += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
