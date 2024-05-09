use std::io::Stdout;
use ratatui::{backend::CrosstermBackend, Terminal};

type Backend = Terminal<CrosstermBackend<Stdout>>;

pub(crate) struct TerminalHandler {
    backend: Backend,
}

impl TerminalHandler {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend
        }
    }
}