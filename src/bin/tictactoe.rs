//! tic-tac-toe solver

use classic::board::{Piece, Board, Move};

enum TTTPiece {
    X, // "X"
    O, // "O"
    E, // " "
}

impl TTTPiece {
    fn opposite(&self) -> TTTPiece {
        match self {
            TTTPiece::X => TTTPiece::O,
            TTTPiece::O => TTTPiece::X,
            _ => TTTPiece::E,
        }
    }
}

impl fmt::Display for TTTPiece {

}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TTTBoard<B: Board> {
    position: Vec<TTTPiece>,
    turn: TTTPiece,
}

impl<B: Board> TTTBoard<B> {
    fn new(position: Vec<TTTPiece>, turn: TTTPiece) -> TTTBoard<B> {
        TTTBoard { position, turn }
    }
    fn turn(&self) -> Piece {
        self.turn
    }
    fn move(&self, location: Move) -> Board<B>:
        let temp_position: Vec<TTTPiece> = self.position.copy();
        temp_position[location] = self.turn;
        TTTBoard { position: temp_position, turn: self._turn.opposite }
    }
    fn legal_moves(&self) -> Vec<Move> {
        [Move(l) for l in range(self.position.len()) if self.position[l] == TTTPiece::E]
    }
    fn is_win(&self) -> bool {
        // three row, three column, and then two diagonal checks
        self.position[0] == self.position[1] && self.position[0] == self.position[2] && self.position[0] != TTTPiece::E ||
        self.position[3] == self.position[4] && self.position[3] == self.position[5] && self.position[3] != TTTPiece::E ||
        self.position[6] == self.position[7] && self.position[6] == self.position[8] && self.position[6] != TTTPiece::E ||
        self.position[0] == self.position[3] && self.position[0] == self.position[6] && self.position[0] != TTTPiece::E ||
        self.position[1] == self.position[4] && self.position[1] == self.position[7] && self.position[1] != TTTPiece::E ||
        self.position[2] == self.position[5] && self.position[2] == self.position[8] && self.position[2] != TTTPiece::E ||
        self.position[0] == self.position[4] && self.position[0] == self.position[8] && self.position[0] != TTTPiece::E ||
        self.position[2] == self.position[4] && self.position[2] == self.position[6] && self.position[2] != TTTPiece::E
    }
    fn evaluate(&self, player: Piece) -> f32 {
        if self.is_win && self.turn == player {
            return -1;
        } else if self.is_win && self.turn != player {
            return 1;
        } else {
            return 0;
        }
    }
}

impl<B: Board> fmt::Display for TTTBoard<B> {
    def __repr__(self) -> str:
        return f"""{self.position[0]}|{self.position[1]}|{self.position[2]}
-----
{self.position[3]}|{self.position[4]}|{self.position[5]}
-----
{self.position[6]}|{self.position[7]}|{self.position[8]}"""
}


fn main() {
    tictactoe();
}
