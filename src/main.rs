pub mod keyboard;
pub mod scripting;
pub mod text;

use std::{io, time::Instant};
use crossterm::terminal::enable_raw_mode;
use scripting::Engine;

fn main() -> io::Result<()> {
    // Enable raw mode on the terminal
    enable_raw_mode().unwrap();

    // Initialise scripting engine
    let mut engine = Engine::new();

    // Tracking variable for timings
    let mut last = Instant::now();

    loop {
        // Delta time
        let now = Instant::now();
        let delta = now.duration_since(last).as_secs_f64();
        last = now;
    }
}