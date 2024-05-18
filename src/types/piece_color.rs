use bevy::prelude::Color;

#[derive(Clone, PartialEq)]
pub enum PieceColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
    Orange,
}

impl PieceColor {
    pub fn choose_piece_color() -> Self {
        use PieceColor::*;
        match rand::random::<u8>() % 7 {
            0 => Red,
            1 => Green,
            2 => Blue,
            3 => Yellow,
            4 => Purple,
            5 => Cyan,
            _ => Orange,
        }
    }

    pub fn get_color(&self) -> Color {
        use PieceColor::*;
        match self {
            Red => Color::rgb(1.0, 0.0, 0.0),
            Green => Color::rgb(0.0, 1.0, 0.0),
            Blue => Color::rgb(0.0, 0.0, 1.0),
            Yellow => Color::rgb(1.0, 1.0, 0.0),
            Purple => Color::rgb(1.0, 0.0, 1.0),
            Cyan => Color::rgb(0.0, 1.0, 1.0),
            Orange => Color::rgb(1.0, 0.5, 0.0),
        }
    }
}
