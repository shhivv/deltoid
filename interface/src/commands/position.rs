use std::str::SplitAsciiWhitespace;

use engine::Game;
use movegen::{Board, Move, MoveParseError, Piece, Square};

// https://github.com/EngineProgramming/honse/blob/master/src/chess/parse_move.rs#LL3C1-L17C2

// The UCI protocol wants castling moves to be reported as king -> 2 steps, such as e1g1, as where cozy-chess wants moves to be reported as king takes rook, such as e1h1.

pub fn parse_move(board: &Board, movestr: &str) -> Result<Move, MoveParseError> {
    let mut mv: Move = movestr.parse()?;

    if board.piece_on(mv.from) == Some(Piece::King) && board.piece_on(mv.to) != Some(Piece::Rook) {
        mv.to = match (mv.from, mv.to) {
            (Square::E1, Square::G1) => Square::H1,
            (Square::E8, Square::G8) => Square::H8,
            (Square::E1, Square::C1) => Square::A1,
            (Square::E8, Square::C8) => Square::A8,
            _ => mv.to,
        };
    }

    Ok(mv)
}

pub fn run(input: &mut SplitAsciiWhitespace, game: &mut Game) {
    match input.next() {
        Some("startpos") => {
            game.reset_position();
            input.next();
        }
        Some("fen") => {
            let fen: String = input
                .take_while(|&part| part != "moves")
                .fold(String::new(), |a, b| a + b + " ");

            game.set_position_from_fen(&fen);
        }
        _ => {}
    };

    let mut moves = vec![];

    for mv in input.by_ref() {
        match parse_move(&game.board, mv) {
            Ok(mv) => moves.push(mv),
            _ => break,
        }
    }

    game.make_moves(moves);
}
