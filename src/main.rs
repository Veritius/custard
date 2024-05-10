pub mod keyboard;
pub mod scripting;
pub mod text;

mod handler;
mod start;
mod state;

use std::io::{self, stdout};
use crossterm::{terminal::*, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    // Configure the terminal and create the abstraction
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Create and run the process handler
    handler::ProcessHandler::new(terminal).run();

    // Reset the terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}