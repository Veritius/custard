#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub foreground: Color,
    pub background: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: None,
        }
    }
}

/// https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}