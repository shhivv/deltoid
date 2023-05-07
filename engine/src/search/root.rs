use crate::pv::PVTable;
use crate::Game;

use crate::defs::{INFINITY, MAX_PLY};
use crate::search::eval::eval;

#[must_use]
pub fn root(
    depth: u8,
    ply: u8,
    game: Game,
    mut alpha: i32,
    beta: i32,
    pv: &mut PVTable,
    nodes: &mut u128,
) -> i32 {
    
    pv.set_length(ply);
    if depth == 0 {
        return eval(game);
    }

    if ply >= MAX_PLY {
        return eval(game);
    }

    let mut max = -INFINITY;
    for mv in game.get_all_moves() {
        *nodes += 1;
        let mut moved = game.clone();
        moved.board.play(mv);
        let score = -root(depth - 1, ply + 1, moved, -alpha, -beta, pv, nodes);

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
