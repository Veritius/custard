use ratatui::{widgets::*, Frame};

/// State for the start menu.
pub(crate) struct StartMenu {

}

impl Default for StartMenu {
    fn default() -> Self {
        Self {

        }
    }
}

impl StartMenu {
    pub fn draw(&self, frame: &mut Frame) {
        let pg = Paragraph::new("")
            .block(Block::bordered().title("custard engine"));

        frame.render_widget(pg, frame.size());
    }
}