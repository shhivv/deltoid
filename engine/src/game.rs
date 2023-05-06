use crate::defs::INFINITY;
use crate::search::root::root;
use movegen::{get_all_moves, Board, Move};

#[derive(Clone)]
pub struct Game {
    pub board: Board,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::startpos(),
        }
    }

    #[must_use]
    pub fn search(&self) -> Move {
        let eval = root(7, self.clone(), -INFINITY, INFINITY);
        eval.1.unwrap()
    }

    pub fn reset_position(&mut self) {
        self.board = Board::startpos();
    }

    pub fn set_position_from_fen(&mut self, fen: &str) {
        self.board = fen.trim().parse().unwrap();
    }

    pub fn make_moves(&mut self, moves: Vec<Move>) {
        for mv in moves {
            self.board.play(mv);
        }
    }

    #[must_use]
    pub fn get_all_moves(&self) -> Vec<Move> {
        get_all_moves(&self.board)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
