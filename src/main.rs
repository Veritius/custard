pub mod keyboard;
pub mod scripting;
pub mod text;

use std::io;
use crossterm::terminal::enable_raw_mode;
use scripting::Engine;

fn main() -> io::Result<()> {
    // Enable raw mode on the terminal
    enable_raw_mode().unwrap();

    // Initialise scripting engine
    let mut engine = Engine::new();

    loop {}
}