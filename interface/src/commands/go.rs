use engine::Game;
use std::str::SplitAsciiWhitespace;

pub fn run(_input: &mut SplitAsciiWhitespace, game: &mut Game) {
    let mv = game.search();
    println!("bestmove {mv}")
}
