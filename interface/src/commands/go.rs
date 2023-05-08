use engine::{Game, search::info::SearchInfo, pv};
use std::str::SplitAsciiWhitespace;

pub fn run(_input: &mut SplitAsciiWhitespace, game: &mut Game) {

    game.info = SearchInfo::new();
    game.pv = pv::PVTable::new();
    
    let mv = game.search();
    println!("bestmove {mv}");
}
