#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub mod game;
pub mod search;
pub use game::Game;
pub mod defs;
