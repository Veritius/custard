use crate::start::StartMenu;

pub(crate) enum State {
    StartMenu(StartMenu),
    InGame,
}