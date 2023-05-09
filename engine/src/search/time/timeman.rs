use movegen::{Board, Color};

use crate::search::options::SearchOptions;

#[must_use]
pub fn timeman(options: &SearchOptions, board: &Board) -> u32 {
    if let SearchOptions::Time(wt, bt, winc, binc, movestogo) = options {
        let (time, inc) = match board.side_to_move() {
            Color::White => (wt, winc.unwrap_or(0)),
            Color::Black => (bt, binc.unwrap_or(0)),
        };
        *time.min(&(*time / (movestogo).unwrap_or(40) + inc)) as u32
    } else {
        unreachable!()
    }
}
