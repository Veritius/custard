#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub foreground: Color,
    pub background: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: Color::WHITE,
            background: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Rgb {
        red: f32,
        blue: f32,
        green: f32,
    },
}

impl Color {
    pub const WHITE: Self = Self::Rgb { red: 1.0, blue: 1.0, green: 1.0 };
    pub const BLACK: Self = Self::Rgb { red: 0.0, blue: 0.0, green: 0.0 };
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}