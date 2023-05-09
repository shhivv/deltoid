use crate::defs::MAX_PLY;
use crate::pv::PVTable;
use crate::search::options::SearchOptions;
use crate::search::root::root;
use crate::search::time::{timeman::timeman, Timer};
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
    pub fn search(&mut self, options: &SearchOptions) -> Move {
        let mut best_move: Option<Move> = None;

        self.info.timer = Timer::new(None);

        let depth = match options {
            SearchOptions::Depth(d) => *d,
            SearchOptions::Time(_, _, _, _, _) => {
                self.info.timer.stop_time = Some(timeman(options, &self.board));
                MAX_PLY
            }
            SearchOptions::MoveTime(t) => {
                self.info.timer.stop_time = Some(*t);
                MAX_PLY
            }
            _ => MAX_PLY,
        };

        for iter in 1..=depth {
            let score = root(
                iter,
                0,
                &self.board,
                -INFINITY,
                INFINITY,
                &mut self.pv,
                &mut self.info,
            );

            if self.info.stop && iter > 1 {
                break;
            }

            let search_time = self.info.timer.start.unwrap().elapsed().as_millis() as u128;

            println!(
                "info depth {iter} score cp {score} nodes {} nps {} time {} pv {}",
                self.info.nodes,
                self.info.nodes / (search_time / 1000).max(1),
                search_time,
                self.pv.display_line(&self.board)
            );

            best_move = self.pv.moves[0][0];

            if let SearchOptions::Nodes(n) = options {
                if self.info.nodes >= *n {
                    break;
                }
            }
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
