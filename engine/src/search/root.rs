use crate::Game;
use movegen::{Color, Move, Piece};
use Color::{Black, White};
use Piece::{Bishop, Knight, Pawn, Queen, Rook};

use crate::defs::INFINITY;
use crate::search::eval::eval;

pub fn root(depth: i16, game: Game, mut alpha: i64, beta: i64) -> (i64, Option<Move>) {
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
                println!("{}", mv);
                alpha = score
            }
        }

        if alpha >= beta {
            break;
        }
    }
    (max, best_move)
}
