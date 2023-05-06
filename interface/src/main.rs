pub mod consts;
pub mod uci;

pub mod commands;

use std::io::{self, stdin};

fn main() -> io::Result<()> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    match input.as_str().trim() {
        "uci" => uci::Interface::new().start()?,
        _ => println!("Unkown protocol"),
    };

    Ok(())
}
