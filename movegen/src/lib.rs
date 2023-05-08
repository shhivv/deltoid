#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

// I'll implement the movegen later so will be just abstracting `cozy-chess` for now
// Reference: https://github.com/EngineProgramming/honse/blob/master/src/chess/move_gen.rs

pub use cozy_chess::{Board, Color, GameStatus, Move, MoveParseError, Piece, Rank, Square};

#[must_use]
pub fn get_all_moves(board: &Board) -> Vec<Move> {
    let mut move_list = vec![];
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });
    move_list
}

#[must_use]
pub fn get_all_captures(board: &Board) -> Vec<Move> {
    let mut captures = vec![];

    let enemy_pieces = board.colors(!board.side_to_move());

    let ep_file = board.en_passant();
    let mut ep_square: Option<Square> = None;

    if let Some(ep) = ep_file {
        if board.side_to_move() == Color::White {
            // En Passant is only possible on the 6th rank for white
            ep_square = Some(Square::new(ep, Rank::Sixth));
        } else {
            // Likewise but on the 3rd rank for black
            ep_square = Some(Square::new(ep, Rank::Third));
        }
    }

    board.generate_moves(|mut moves| {
        let mut capturable = enemy_pieces;

        if let Some(ep_possible) = ep_square {
            if moves.piece == Piece::Pawn {
                capturable |= ep_possible.bitboard();
            }
        }

        moves.to &= capturable;

        captures.extend(moves);
        false
    });

    captures
}

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

#[must_use]
pub fn move_to_string(board: &Board, mv: Move) -> String {
    if board.piece_on(mv.from) == Some(Piece::King) {
        return match (mv.from, mv.to) {
            (Square::E1, Square::H1) => String::from("e1g1"),
            (Square::E8, Square::H8) => String::from("e8g8"),
            (Square::E1, Square::A1) => String::from("e1c1"),
            (Square::E8, Square::A8) => String::from("e8c8"),
            _ => mv.to_string(),
        };
    }

    mv.to_string()
}
