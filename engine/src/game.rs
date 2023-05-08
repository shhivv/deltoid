use crate::pv::PVTable;
use crate::search::root::root;
use crate::{defs::INFINITY, search::info::SearchInfo};
use movegen::{get_all_moves, Board, Move};

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub info: SearchInfo,
    pub pv: PVTable,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::startpos(),
            info: SearchInfo::new(),
            pv: PVTable::new(),
        }
    }

    #[must_use]
    pub fn search(&mut self) -> Move {
        let mut best_move: Option<Move> = None;

        let depth = 7;

        for depth in 1..=depth {
            let score = root(
                depth,
                0,
                &self.board,
                -INFINITY,
                INFINITY,
                &mut self.pv,
                &mut self.info,
            );

            println!(
                "info depth {depth} score cp {score} nodes {} pv {}",
                self.info.nodes,
                self.pv.display_line(&self.board)
            );

            best_move = self.pv.moves[0][0];
        }
        best_move.unwrap()
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
