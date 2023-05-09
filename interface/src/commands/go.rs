use engine::{
    pv,
    search::{info::SearchInfo, options::SearchOptions},
    Game,
};
use std::str::SplitAsciiWhitespace;

pub fn run(input: &mut SplitAsciiWhitespace, game: &mut Game) {
    game.info = SearchInfo::new();
    game.pv = pv::PVTable::new();

    let options = SearchOptions::parse(input);
    let mv = game.search(&options);

    println!("bestmove {mv}");
}
