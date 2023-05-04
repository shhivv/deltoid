#![allow(unused)]
// Reference: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

use engine::Game;

use crate::{
    commands::{go, position},
    consts::{AUTHOR, NAME},
};
use std::io::{self, stdin};

pub struct Interface {}

impl Interface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) -> io::Result<()> {
        Self::intro();

        let mut game = Game::new();

        loop {
            let mut raw = String::new();
            match stdin().read_line(&mut raw) {
                Ok(0) => break,
                Ok(_) => {}
                Err(_) => break,
            }

            let mut input = raw.split_ascii_whitespace();

            match input.next().unwrap_or("") {
                "ucinewgame" => game = Game::new(),
                "isready" => println!("readyok"),
                "quit" => break,
                "position" => position::run(&mut input, &mut game),
                "go" => go::run(&mut input, &mut game),
                _ => {}
            }
        }

        Ok(())
    }

    fn intro() {
        println!("id name {}", NAME);
        println!("id author {}", AUTHOR);

        Self::options();

        println!("uciok");
    }

    fn options() {}
}

impl Default for Interface {
    fn default() -> Self {
        Self::new()
    }
}
