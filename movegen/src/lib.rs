// I'll implement the movegen later so will be just abstracting `cozy-chess` for now
// Reference: https://github.com/EngineProgramming/honse/blob/master/src/chess/move_gen.rs

pub use cozy_chess::{Board, Color, Move, MoveParseError, Piece, Rank, Square};

pub fn get_all_moves(board: &Board) -> Vec<Move> {
    let mut move_list = vec![];
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });
    move_list
}

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
