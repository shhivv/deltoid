use movegen::{get_all_captures, Board, GameStatus};

use crate::defs::MAX_PLY;

use super::super::defs::DRAW;
use super::eval::end::mated_in;
use super::{eval::eval, info::SearchInfo};

pub fn quiescence(
    mut alpha: i32,
    beta: i32,
    board: &Board,
    ply: u8,
    info: &mut SearchInfo,
    // tt: &mut TranspositionTable,
) -> i32 {
    if info.nodes % 1024 == 0 {
        if let (Some(start), Some(end)) = (info.timer.start, info.timer.stop_time) {
            #[allow(clippy::cast_possible_truncation)]
            if start.elapsed().as_millis() as u32 >= end {
                info.stop = true;
            }
        }
    }

    if info.stop && ply > 0 {
        return 0;
    }

    if ply >= MAX_PLY {
        if !board.checkers().is_empty() {
            return 0;
        }
        return eval(board);
    }

    match board.status() {
        GameStatus::Won => return mated_in(ply),
        GameStatus::Drawn => return DRAW,
        GameStatus::Ongoing => (),
    };

    let stand_pat = eval(board);
    if stand_pat >= beta {
        return beta;
    };

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let captures = get_all_captures(board);
    for mv in captures {
        info.nodes += 1;
        let mut moved = board.clone();
        moved.play_unchecked(mv);

        let score = -quiescence(-beta, -alpha, &moved, ply + 1, info);

        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
