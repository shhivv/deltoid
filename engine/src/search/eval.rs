use crate::Game;
use movegen::{Color, Move, Piece};
use Color::{Black, White};
use Piece::{Bishop, Knight, Pawn, Queen, Rook};

// PeSTO's Evaluation Function
// https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

pub fn eval(game: Game) -> i64 {
    let multiplier = match game.board.side_to_move() {
        Black => -1i64,
        White => 1,
    };

    let black = game.board.colors(Black).len();
    let white = game.board.colors(White).len();

    let b = game.board;

    let material_score: i64 = (9
        * (b.colored_pieces(White, Queen).len() as i64
            - b.colored_pieces(Black, Queen).len() as i64)
        + 5 * (b.colored_pieces(White, Rook).len() as i64
            - b.colored_pieces(Black, Rook).len() as i64)
        + 3 * (b.colored_pieces(White, Knight).len() as i64
            - b.colored_pieces(Black, Knight).len() as i64)
        + 3 * (b.colored_pieces(White, Bishop).len() as i64
            - b.colored_pieces(Black, Bishop).len() as i64)
        + (b.colored_pieces(White, Pawn).len() as i64
            - b.colored_pieces(Black, Pawn).len() as i64));

    material_score * (white as i64) - (black as i64) * multiplier
}
