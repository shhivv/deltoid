use crate::defs::INFINITY;
use crate::pv::PVTable;
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
        let mut pv_table = PVTable::new();
        let mut nodes = 0u128;

        for depth in 1..7 {
            let score = root(
                depth,
                0,
                self.clone(),
                -INFINITY,
                INFINITY,
                &mut pv_table,
                &mut nodes,
            );

            println!(
                "info depth {depth} score cp {score} nodes {nodes} pv {}",
                pv_table.display_line(&self.board)
            );
        }

        pv_table.moves[0][0].unwrap()
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
