#![allow(clippy::cast_possible_truncation)]

use movegen::Move;

// Inspired by https://github.com/SnowballSH/Iceburn/blob/2.0/src/tt.rs
#[derive(Clone)]
pub enum TTFlag {
    Exact = 0,
    Alpha = 1,
    Beta = 2,
    Empty = -1,
}

#[derive(Clone)]
pub struct TTEntry {
    pub key: u16,
    pub score: i32,
    pub bestmove: Option<Move>,
    pub depth: u8,
    pub spec: TTFlag,
}

impl TTEntry {
    #[must_use]
    pub const fn new(
        hash: u64,
        score: i32,
        bestmove: Option<Move>,
        depth: u8,
        spec: TTFlag,
    ) -> Self {
        Self {
            key: (hash >> 48) as u16,
            score,
            bestmove,
            depth,
            spec,
        }
    }

    #[must_use]
    pub const fn is_key_valid(&self, hash: u64) -> bool {
        self.key == (hash >> 48) as u16
    }
}

impl Default for TTEntry {
    fn default() -> Self {
        Self {
            key: 0,
            score: 0,
            bestmove: None,
            depth: 0,
            spec: TTFlag::Empty,
        }
    }
}

#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<TTEntry>,
    size: usize,
}

impl TranspositionTable {
    #[must_use]
    pub fn with_size(size_mb: u64) -> Self {
        let hash_size = 0x0010_0000 * size_mb;
        let tt_entry_size = std::mem::size_of::<TTEntry>() as u64;
        let limit = hash_size / tt_entry_size;

        // Create a Vector with the max possible entry capacity
        let table = vec![TTEntry::default(); limit as usize];

        Self {
            table,
            size: limit as usize,
        }
    }

    #[must_use]
    pub fn get(&self, hash: u64) -> Option<&TTEntry> {
        unsafe {
            let res = self.table.get_unchecked(hash as usize % self.size);
            if res.is_key_valid(hash) {
                Some(res)
            } else {
                None
            }
        }
    }

    pub fn insert(&mut self, hash: u64, entry: TTEntry) {
        self.table[hash as usize % self.size] = entry;
    }
}
