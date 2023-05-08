pub mod end;
pub mod pst;

use lazy_static::lazy_static;
use movegen::{Board, Color};

use pst::PieceSquareTable;

lazy_static! {
    static ref PST: PieceSquareTable = PieceSquareTable::new();
}

// PeSTO's Evaluation Function
// https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

pub const GAME_PHASE_INC: [i32; 12] = [0, 0, 1, 1, 1, 1, 2, 2, 4, 4, 0, 0];

#[allow(clippy::similar_names)]
#[must_use]
pub fn eval(board: &Board) -> i32 {
    let mut game_phase = 0;
    let mut mg = 0;
    let mut eg = 0;

    for sq in board.occupied() {
        // square
        let piece = board.piece_on(sq).unwrap();
        let color = board.color_on(sq).unwrap();

        let sq_usize = sq as usize;
        let pc = (piece as usize * 2) + color as usize;

        match color {
            Color::White => {
                mg += PST.mg_table[pc][sq_usize];
                eg += PST.eg_table[pc][sq_usize];
            }
            Color::Black => {
                mg -= PST.mg_table[pc][sq_usize];
                eg -= PST.eg_table[pc][sq_usize];
            }
        }
        game_phase += GAME_PHASE_INC[pc];
    }

    let mg_phase = game_phase.min(24);
    let eg_phase = 24 - mg_phase;

    let score = (mg * mg_phase + eg * eg_phase) / 24;

    //negamax
    match board.side_to_move() {
        Color::White => score,
        Color::Black => -score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::Board;

    #[test]
    fn eval_startpos() {
        let board = Board::startpos();
        assert_eq!(eval(&board), 0);
    }

    #[test]
    fn eval_white() {
        let board = Board::from_fen(
            "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 1",
            false,
        )
        .unwrap();
        assert_eq!(eval(&board), -25);
    }

    #[test]
    fn eval_black() {
        let board = Board::from_fen(
            "rnb1kbnr/pppp1ppp/8/4pq2/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 1",
            false,
        )
        .unwrap();
        assert_eq!(eval(&board), 39);
    }
}
