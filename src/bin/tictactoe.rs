//! tic-tac-toe solver
use std::fmt;

extern crate classic;
use classic::board::{Board, Move, Piece};
use classic::minimax::find_best_move;
use text_io::read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TTTPiece {
    X, // "X"
    O, // "O"
    E, // " "
}

impl Piece for TTTPiece {
    fn opposite(&self) -> TTTPiece {
        match self {
            TTTPiece::X => TTTPiece::O,
            TTTPiece::O => TTTPiece::X,
            _ => TTTPiece::E,
        }
    }
}

impl fmt::Display for TTTPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TTTPiece::X => "X",
                TTTPiece::O => "O",
                TTTPiece::E => " ",
            }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TTTBoard {
    position: Vec<TTTPiece>,
    turn: TTTPiece,
}

impl Board<TTTPiece> for TTTBoard {
    fn turn(&self) -> TTTPiece {
        self.turn
    }
    fn make_move(&self, location: Move) -> TTTBoard {
        let mut temp_position: Vec<TTTPiece> = self.position.clone();
        temp_position[location as usize] = self.turn;
        TTTBoard {
            position: temp_position,
            turn: self.turn.opposite(),
        }
    }
    fn legal_moves(&self) -> Vec<Move> {
        (0..self.position.len())
            .filter(|&i| self.position[i] == TTTPiece::E)
            .map(|x| x as Move)
            .collect::<Vec<_>>()
        //[Move(l) for l in range(self.position.len()) if self.position[l] == TTTPiece::E]
    }
    fn is_win(&self) -> bool {
        // three row, three column, and then two diagonal checks
        self.position[0] == self.position[1]
            && self.position[0] == self.position[2]
            && self.position[0] != TTTPiece::E
            || self.position[3] == self.position[4]
                && self.position[3] == self.position[5]
                && self.position[3] != TTTPiece::E
            || self.position[6] == self.position[7]
                && self.position[6] == self.position[8]
                && self.position[6] != TTTPiece::E
            || self.position[0] == self.position[3]
                && self.position[0] == self.position[6]
                && self.position[0] != TTTPiece::E
            || self.position[1] == self.position[4]
                && self.position[1] == self.position[7]
                && self.position[1] != TTTPiece::E
            || self.position[2] == self.position[5]
                && self.position[2] == self.position[8]
                && self.position[2] != TTTPiece::E
            || self.position[0] == self.position[4]
                && self.position[0] == self.position[8]
                && self.position[0] != TTTPiece::E
            || self.position[2] == self.position[4]
                && self.position[2] == self.position[6]
                && self.position[2] != TTTPiece::E
    }
    fn evaluate(&self, player: TTTPiece) -> f32 {
        if self.is_win() && self.turn == player {
            return -1.0;
        } else if self.is_win() && self.turn != player {
            return 1.0;
        } else {
            return 0.0;
        }
    }
}

impl fmt::Display for TTTBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            self.position[0],
            self.position[1],
            self.position[2],
            self.position[3],
            self.position[4],
            self.position[5],
            self.position[6],
            self.position[7],
            self.position[8]
        )
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
    let mut board = TTTBoard {
        position: (0..9).map(|_| TTTPiece::E).collect(),
        turn: TTTPiece::X,
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
            TTTPiece::X,
            TTTPiece::O,
            TTTPiece::X,
            TTTPiece::X,
            TTTPiece::E,
            TTTPiece::O,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::O,
        ];
        let test_board1: TTTBoard = TTTBoard {
            position: to_win_easy_position,
            turn: TTTPiece::X,
        };
        let answer1 = find_best_move(test_board1, 2);
        assert_eq!(answer1, 6);
    }

    #[test]
    fn test_block_position() {
        // must block O's win
        let to_block_position = vec![
            TTTPiece::X,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::O,
            TTTPiece::E,
            TTTPiece::X,
            TTTPiece::O,
        ];
        let test_board2 = TTTBoard {
            position: to_block_position,
            turn: TTTPiece::X,
        };
        let answer2 = find_best_move(test_board2, 2);
        assert_eq!(answer2, 2);
    }

    #[test]
    fn test_hard_position() {
        // find the best move to win 2 moves
        let to_win_hard_position = vec![
            TTTPiece::X,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::O,
            TTTPiece::O,
            TTTPiece::X,
            TTTPiece::E,
        ];
        let test_board3: TTTBoard = TTTBoard {
            position: to_win_hard_position,
            turn: TTTPiece::X,
        };
        let answer3 = find_best_move(test_board3, 2);
        assert_eq!(answer3, 1);
    }
}
