//! tic-tac-toe solver
use std::fmt;

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
    position: Vec<u8>,
    turn: OwarePiece,
    score1: u8,
    score2: u8,
}

impl Board<OwarePiece> for OwareBoard {
    fn turn(&self) -> OwarePiece {
        self.turn
    }
    fn make_move(&self, location: Move) -> OwareBoard {
        let n_to_sow = self.position[location as usize] as usize; // num seeds to sow
        let houses = (0..n_to_sow).map(|x| (x + location as usize) % 12).collect::<Vec<usize>>();
        let mut temp_position: Vec<u8> = self.position.clone();
        for h in &houses { // sow
            temp_position[h] += 1;
        }
        let mut b = OwareBoard {
            position: temp_position,
            turn: self.turn.opposite(),
            score1: self.score1,
            score2: self.score2,
        };
        for &h in houses.iter().rev() { // score
            let num_before = self.position[h];
            if num_before == 1 || num_before == 2 {
                if self.turn == OwarePiece::P1 && h >= 6 {
                    b.score1 += num_before + 1;
                    b.position[h] = 0;
                } else if self.turn == OwarePiece::P2 && h < 6 {
                    b.score2 += num_before + 1;
                    b.position[h] = 0;
                } else {
                    break;
                }
            }
        }
        b
    }
    fn legal_moves(&self) -> Vec<Move> {
        let (min, max) = match self.turn {
            OwarePiece::P1 => (0,6),
            OwarePiece::P2 => (6,12),
        };
        (min..max).filter(|&x| self.position[x] > 0) // on player side with seeds in house
            .map(|x| x as Move)
            .collect::<Vec<_>>()
    }
    fn is_win(&self) -> bool {
        // three row, three column, and then two diagonal checks
        self.score1 > 24 || self.score2 > 24
    }
    fn evaluate(&self, player: OwarePiece) -> f32 {
        (self.score1 - self.score2) as f32
    }
}

impl fmt::Display for OwareBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:2} | {:2} {:2} {:2} {:2} {:2} {:2}\n     {:2} {:2} {:2} {:2} {:2} {:2} | {:2}",
            self.score2,
            self.position[0],
            self.position[1],
            self.position[2],
            self.position[3],
            self.position[4],
            self.position[5],
            self.position[6],
            self.position[7],
            self.position[8],
            self.position[9],
            self.position[10],
            self.position[11],
            self.score1
        )
    }
}

fn get_player_move<B: Board<P>, P: Piece>(board: &B) -> Move {
    loop {
        let line: String = read!("move> {}\n");
        let player_move: Move = line.parse::<Move>().unwrap();
        if board.legal_moves().contains(&player_move) {
            return player_move;
        }
    }
}

fn main() {
    // main game loop
    let mut board = OwareBoard {
        position: (0..12).map(|_| 4).collect(),
        turn: OwarePiece::P1,
        score1: 0,
        score2: 0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use classic::minimax::find_best_move;
    #[test]
    fn test_easy_position() {
        // win in 1 move
        let to_win_easy_position = vec![
            OwarePiece::X,
            OwarePiece::O,
            OwarePiece::X,
            OwarePiece::X,
            OwarePiece::E,
            OwarePiece::O,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::O,
        ];
        let test_board1: OwareBoard = OwareBoard {
            position: to_win_easy_position,
            turn: OwarePiece::X,
        };
        let answer1 = find_best_move(test_board1, 2);
        assert_eq!(answer1, 6);
    }

    #[test]
    fn test_block_position() {
        // must block O's win
        let to_block_position = vec![
            OwarePiece::X,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::O,
            OwarePiece::E,
            OwarePiece::X,
            OwarePiece::O,
        ];
        let test_board2 = OwareBoard {
            position: to_block_position,
            turn: OwarePiece::X,
        };
        let answer2 = find_best_move(test_board2, 2);
        assert_eq!(answer2, 2);
    }

    #[test]
    fn test_hard_position() {
        // find the best move to win 2 moves
        let to_win_hard_position = vec![
            OwarePiece::X,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::E,
            OwarePiece::O,
            OwarePiece::O,
            OwarePiece::X,
            OwarePiece::E,
        ];
        let test_board3: OwareBoard = OwareBoard {
            position: to_win_hard_position,
            turn: OwarePiece::X,
        };
        let answer3 = find_best_move(test_board3, 2);
        assert_eq!(answer3, 1);
    }
}
