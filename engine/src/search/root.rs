use crate::pv::PVTable;
use movegen::{get_all_moves, Board};

use crate::defs::{DRAW, INFINITY, MAX_PLY};
use crate::search::eval::eval;
use movegen::GameStatus;

use super::eval::end::mated_in;
use super::info::SearchInfo;

#[must_use]
pub fn root(
    depth: u8,
    ply: u8,
    board: &Board,
    mut alpha: i32,
    beta: i32,
    pv: &mut PVTable,
    info: &mut SearchInfo,
) -> i32 {

    if info.nodes % 1024 == 0 {
        if let (Some(start), Some(end)) = (info.timer.start, info.timer.stop_time) {
            #[allow(clippy::cast_possible_truncation)]
            if start.elapsed().as_millis() as u32 >= end {
                info.stop = true;
            }
        }
    }

    pv.set_length(ply);

    if info.stop && ply > 0 {
        return 0;
    }

    // We have exceeded the maximum search requirements.
    if depth == 0 || ply >= MAX_PLY {
        return eval(board);
    }

    match board.status() {
        GameStatus::Won => return mated_in(ply),
        GameStatus::Drawn => return DRAW,
        GameStatus::Ongoing => (),
    };

    let mut max = -INFINITY;

    for mv in get_all_moves(board) {
        info.nodes += 1;

        let mut moved = board.clone();
        moved.play_unchecked(mv);

        let score = -root(depth - 1, ply + 1, &moved, -beta, -alpha, pv, info);

        if score > max {
            max = score;
            if score > alpha {
                alpha = score;
                pv.insert(ply, mv);
            }
        }

        if alpha >= beta {
            break;
        }
    }

    max
}
