
type Move = i32;

trait Piece {
    fn opposite(&self) -> Piece;
}

trait Board<P: Piece> {
    fn turn(&self) -> P;
    fn move(&self, location: Move) -> Board;
    fn legal_moves(&self) -> Vec<Move>;
    fn is_win(&self) -> bool;
    fn is_draw(&self) -> bool {
        !self.is_win() && self.legal_moves().len() == 0
    }
    fn evaluate(&self, player: P) - f32;
}
