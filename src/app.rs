use std::{io::{self, Stdout}, time::Duration};
use crossterm::event;
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::{start::StartMenu, state::State};

pub(super) type Backend = Terminal<CrosstermBackend<Stdout>>;

pub(crate) struct App {
    backend: Backend,
    state: State,
}

impl App {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            state: State::StartMenu(StartMenu::default()),
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