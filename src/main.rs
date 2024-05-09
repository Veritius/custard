pub mod keyboard;
pub mod scripting;
pub mod text;

use std::io;
use crossterm::{terminal::*, ExecutableCommand};
use scripting::Engine;

fn main() -> io::Result<()> {
    // Initialise scripting engine
    let mut engine = Engine::new();

    // Configure the terminal
    enable_raw_mode().unwrap();
    io::stdout().execute(EnterAlternateScreen)?;

    loop {}

    // Reset the terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}