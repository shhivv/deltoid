// Reference: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

use engine::Game;
use std::process::exit;

use crate::{
    commands::{go, position},
    consts::{AUTHOR, NAME},
};
use std::io::stdin;

pub struct Interface {}

impl Interface {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {
        Self::intro();

        let mut game = Game::new();

        loop {
            let mut raw = String::new();
            match stdin().read_line(&mut raw) {
                Ok(0) | Err(_) => break,
                Ok(_) => {}
            }

            let mut input = raw.split_ascii_whitespace();

            match input.next().unwrap_or("") {
                "ucinewgame" => game = Game::new(),
                "isready" => println!("readyok"),
                "quit" => exit(0),
                "position" => position::run(&mut input, &mut game),
                "go" => go::run(&mut input, &mut game),
                _ => {}
            }
        }
    }

    fn intro() {
        println!("id name {}", NAME);
        println!("id author {}", AUTHOR);

        Self::options();

        println!("uciok");
    }

    const fn options() {}
}

impl Default for Interface {
    fn default() -> Self {
        Self::new()
    }
}
