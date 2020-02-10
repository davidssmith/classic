use crate::board::{Board, Move, Piece};

fn f32_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
fn f32_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

// Find the best possible outcome for original player
fn minimax<B: Board<P>, P: Piece>(
    board: B,
    maximizing: bool,
    original_player: P,
    max_depth: i32,
) -> f32 {
    // Base case – terminal position or maximum depth reached
    if board.is_win() || board.is_draw() || max_depth == 0 {
        return board.evaluate(original_player);
    }
    // Recursive case - maximize your gains or minimize the opponent's gains
    if maximizing {
        let mut best_eval = f32::NEG_INFINITY; // arbitrarily low starting point
        for m in board.legal_moves() {
            let result = minimax(board.make_move(m), false, original_player, max_depth - 1);
            best_eval = f32_max(result, best_eval); // we want the move with the highest evaluation
        }
        return best_eval;
    } else {
        // minimizing
        let mut worst_eval = f32::INFINITY;
        for m in board.legal_moves() {
            let result = minimax(board.make_move(m), true, original_player, max_depth - 1);
            worst_eval = f32_min(result, worst_eval); // we want the move with the lowest evaluation
        }
        return worst_eval;
    }
}

fn alphabeta<B: Board<P>, P: Piece>(
    board: B,
    maximizing: bool,
    original_player: P,
    max_depth: i32,
    alpha: f32,
    beta: f32,
) -> f32 {
    // defaults: max_depth=8, alpha=-inf, beta=inf
    // Base case – terminal position or maximum depth reached
    if board.is_win() || board.is_draw() || max_depth == 0 {
        return board.evaluate(original_player);
    }
    // Recursive case - maximize your gains or minimize the opponent's gains
    if maximizing {
        for m in board.legal_moves() {
            let result = alphabeta(
                board.make_move(m),
                false,
                original_player,
                max_depth - 1,
                alpha,
                beta,
            );
            let alpha = f32_max(result, alpha);
            if beta <= alpha {
                break;
            }
        }
        return alpha;
    } else {
        // minimizing
        for m in board.legal_moves() {
            let result = alphabeta(
                board.make_move(m),
                true,
                original_player,
                max_depth - 1,
                alpha,
                beta,
            );
            let beta = f32_min(result, beta);
            if beta <= alpha {
                break;
            }
        }
        return beta;
    }
}

// Find the best possible move in the current position
// looking up to max_depth ahead
pub fn find_best_move<B: Board<P>, P: Piece>(board: B, max_depth: i32) -> Move {
    // default: max_depth=8
    let mut best_eval = f32::NEG_INFINITY;
    let mut best_move: Move = -1 as Move;
    for &m in board.legal_moves().iter() {
        let result = alphabeta(
            board.make_move(m),
            false,
            board.turn(),
            max_depth,
            f32::NEG_INFINITY,
            f32::INFINITY,
        );
        if result > best_eval {
            best_eval = result;
            best_move = m;
        }
    }
    best_move
}
