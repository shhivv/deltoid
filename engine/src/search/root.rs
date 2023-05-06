use crate::Game;
use movegen::Move;

use crate::defs::INFINITY;
use crate::search::eval::eval;

#[must_use]
pub fn root(depth: i16, game: Game, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
    let mut best_move: Option<Move> = None;
    if depth == 0 {
        return (eval(game), best_move);
    }

    let mut max = -INFINITY;
    for mv in game.get_all_moves() {
        let mut moved = game.clone();
        moved.board.play(mv);
        let score = -(root(depth - 1, moved, -alpha, -beta).0);

        if score > max {
            max = score;
            best_move = Some(mv);
            if score > alpha {
                alpha = score;
            }
        }

        if alpha >= beta {
            break;
        }
    }
    (max, best_move)
}
