use std::io::Stdout;
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::scripting::Engine;

type Backend = Terminal<CrosstermBackend<Stdout>>;

pub(crate) struct ProcessHandler {
    backend: Backend,
    engine: Engine,
}

impl ProcessHandler {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            engine: Engine::new(),
        }
    }

    pub fn run(mut self) {

    }
}