pub mod keyboard;
pub mod scripting;
pub mod text;

mod handler;
mod start;
mod state;

use std::io::{self, stdout};
use crossterm::{terminal::*, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, Terminal};
use scripting::Engine;

fn main() -> io::Result<()> {
    // Initialise scripting engine
    let mut engine = Engine::new();

    // Configure the terminal and create the abstraction
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Create and run the terminal handler
    let mut handler = handler::TerminalHandler::new(terminal);
    todo!();

    // Reset the terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}