pub mod keyboard;
pub mod scripting;
pub mod text;

mod app;
mod layout;
mod start;
mod state;

use std::io::{self, stdout};
use app::Backend;
use crossterm::{execute, terminal::*};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    // Add panic hook to reset stdout on panic
    init_panic_hook();

    // Configure terminal and create process handler
    let terminal = init_tui()?;
    app::App::new(terminal).run()?;

    // Reset the terminal
    exit_tui()?;

    Ok(())
}

fn init_panic_hook() {
    use std::panic::*;
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = exit_tui();
        original_hook(panic_info);
    }));
}

fn init_tui() -> io::Result<Backend> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

fn exit_tui() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}