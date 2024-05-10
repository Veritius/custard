use std::{io::{self, Stdout}, time::Duration};
use crossterm::event;
use ratatui::{backend::CrosstermBackend, Terminal};
use rhai::Scope;
use crate::{scripting::Engine, start::StartMenu, state::State};

type Backend = Terminal<CrosstermBackend<Stdout>>;

pub(crate) struct App {
    backend: Backend,
    engine: Engine,
    state: State,

    savegame: Option<Scope<'static>>,
}

impl App {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            engine: Engine::new(),
            state: State::StartMenu(StartMenu::default()),

            savegame: None,
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        loop {
            self.backend.draw(|frame| {
                match &mut self.state {
                    State::StartMenu(menu) => {
                        menu.draw(frame);
                    },
                    State::InGame => todo!(),
                }
            })?;

            if event::poll(Duration::from_millis(16))? {
                // do nothing
            }
        }
    }
}