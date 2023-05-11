use engine::{
    pv,
    search::{info::SearchInfo, options::SearchOptions},
    ttable::TranspositionTable,
    Game,
};
use std::str::SplitAsciiWhitespace;

pub fn run(input: &mut SplitAsciiWhitespace, game: &mut Game) {
    game.info = SearchInfo::new();
    game.pv = pv::PVTable::new();
    game.tt = TranspositionTable::with_size(64);

    let options = SearchOptions::parse(input);
    let mv = game.search(&options);

    println!("bestmove {mv}");
}
