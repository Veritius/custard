pub mod keyboard;
pub mod text;

use std::io;
use crossterm::terminal::enable_raw_mode;

fn main() -> io::Result<()> {
    enable_raw_mode().unwrap();

    loop {}
}