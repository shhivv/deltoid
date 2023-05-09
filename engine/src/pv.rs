use crate::defs::MAX_PLY;
use movegen::{move_to_string, Board, Move};

// Implementation based on https://www.youtube.com/watch?v=LOR-dkAkUyM

#[derive(Clone)]
pub struct PVTable {
    pub moves: [[Option<Move>; MAX_PLY as usize]; MAX_PLY as usize],
    pub length: [u8; MAX_PLY as usize],
}

impl PVTable {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            moves: [[None; MAX_PLY as usize]; MAX_PLY as usize],
            length: [0; MAX_PLY as usize],
        }
    }

    pub fn set_length(&mut self, ply: u8) {
        self.length[ply as usize] = ply;
    }

    pub fn insert(&mut self, ply: u8, mv: Move) {
        self.moves[ply as usize][ply as usize] = Some(mv);
        let mut next_ply = ply + 1;

        if !next_ply == MAX_PLY {
            while next_ply < self.length[(ply as usize) + 1] {
                self.moves[ply as usize][next_ply as usize] =
                    self.moves[(ply as usize) + 1][next_ply as usize];
                next_ply += 1;
            }
            self.length[ply as usize] = self.length[(ply as usize) + 1];
        }
    }

    #[must_use]
    pub fn display_line(&self, board: &Board) -> String {
        let mut line = String::new();

        for mv in self.moves[0].into_iter().flatten() {
            line.push_str(&format!("{} ", move_to_string(board, mv)));
        }
        line
    }
}

impl Default for PVTable {
    fn default() -> Self {
        Self::new()
    }
}
