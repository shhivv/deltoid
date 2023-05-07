use std::str::SplitAsciiWhitespace;

use engine::Game;
use movegen::parse_move;

// https://github.com/EngineProgramming/honse/blob/master/src/chess/parse_move.rs#LL3C1-L17C2

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
