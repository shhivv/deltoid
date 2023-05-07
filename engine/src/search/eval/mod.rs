pub mod pst;

use crate::Game;
use lazy_static::lazy_static;
use movegen::Color;

use pst::PieceSquareTable;

lazy_static! {
    static ref PST: PieceSquareTable = PieceSquareTable::new();
}

// PeSTO's Evaluation Function
// https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

pub const GAME_PHASE_INC: [i32; 12] = [0, 0, 1, 1, 1, 1, 2, 2, 4, 4, 0, 0];

#[allow(clippy::similar_names)]
#[must_use]
pub fn eval(game: Game) -> i32 {
    let board = game.board;

    let mut game_phase = 0;
    let mut mg = 0;
    let mut eg = 0;

    for sq in board.occupied() {
        // square
        let piece = board.piece_on(sq).unwrap() as usize;
        let color = board.color_on(sq).unwrap() as usize;

        let sq_usize = sq as usize;
        let pc = (piece * 2) + color;

        mg += PST.mg_table[pc][sq_usize];
        eg += PST.eg_table[pc][sq_usize];
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
