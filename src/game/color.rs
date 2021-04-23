#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opp_color(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
