use crate::pv::PVTable;
use crate::ttable::{TTEntry, TTFlag, TranspositionTable};
use movegen::{get_all_moves, Board};

use crate::defs::{DRAW, INFINITY, MAX_PLY};
use crate::search::eval::eval;
use movegen::GameStatus;

use super::eval::end::mated_in;
use super::info::SearchInfo;
use super::quiescence::quiescence;

#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn root(
    depth: u8,
    ply: u8,
    board: &Board,
    mut alpha: i32,
    beta: i32,
    pv: &mut PVTable,
    info: &mut SearchInfo,
    tt: &mut TranspositionTable,
) -> i32 {
    let mut max = -INFINITY;

    let mut best_move = None;

    if info.nodes % 1024 == 0 {
        if let (Some(start), Some(end)) = (info.timer.start, info.timer.stop_time) {
            #[allow(clippy::cast_possible_truncation)]
            if start.elapsed().as_millis() as u32 >= end {
                info.stop = true;
            }
        }
    }

    let mut flag = TTFlag::Alpha;

    let hash = board.hash();
    if let Some(entry) = tt.get(hash) {
        if entry.depth >= depth {
            let score = match entry.spec {
                TTFlag::Exact => Some(entry.score),
                TTFlag::Alpha => {
                    if entry.score <= alpha {
                        Some(alpha)
                    } else {
                        None
                    }
                }
                TTFlag::Beta => {
                    if entry.score >= beta {
                        Some(beta)
                    } else {
                        None
                    }
                }
                TTFlag::Empty => None,
            };

            if let Some(final_score) = score {
                return final_score;
            }
        }
    }

    pv.set_length(ply);

    if info.stop && ply > 0 {
        return 0;
    }

    // We have exceeded the maximum search requirements.
    if ply >= MAX_PLY {
        return eval(board);
    }

    if depth == 0 {
        return quiescence(alpha, beta, board, ply, info);
    }

    match board.status() {
        GameStatus::Won => return mated_in(ply),
        GameStatus::Drawn => return DRAW,
        GameStatus::Ongoing => (),
    };

    for mv in &get_all_moves(board) {
        if best_move == None {
            best_move = Some(*mv);
        }

        info.nodes += 1;

        let mut moved = board.clone();
        moved.play_unchecked(*mv);

        let score = -root(depth - 1, ply + 1, &moved, -beta, -alpha, pv, info, tt);

        if score >= beta {
            tt.insert(
                hash,
                TTEntry::new(hash, beta, best_move, depth, TTFlag::Beta),
            );
            return beta;
        }

        if score > max {
            max = score;
            if score > alpha {
                best_move = Some(*mv);
                flag = TTFlag::Exact;
                alpha = score;
                if board.is_legal(*mv){
                pv.insert(ply, *mv);
                }
            }
        }
    }

    tt.insert(hash, TTEntry::new(hash, alpha, best_move, depth, flag));

    alpha
}
