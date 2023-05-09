#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub mod consts;
pub mod uci;

pub mod commands;

use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    match input.as_str().trim() {
        "uci" => uci::Interface::new().start(),
        _ => println!("Unkown protocol"),
    };
}
