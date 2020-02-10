
pub type Move = i32;

pub trait Piece {
    fn opposite(&self) -> dyn Piece;
}

pub trait Board<P: Piece> {
    fn turn(&self) -> P;
    fn make_move(&self, location: Move) -> dyn Board<P>;
    fn legal_moves(&self) -> Vec<Move>;
    fn is_win(&self) -> bool;
    fn is_draw(&self) -> bool {
        !self.is_win() && self.legal_moves().len() == 0
    }
    fn evaluate(&self, player: P) -> f32;
}
