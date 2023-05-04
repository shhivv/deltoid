pub mod consts;
pub mod interface;

pub mod commands;

use std::io::{self, stdin};

fn main() -> io::Result<()> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    match input.as_str().trim() {
        "uci" => interface::Interface::new().start()?,
        _ => println!("Unkown protocol"),
    };

    Ok(())
}
