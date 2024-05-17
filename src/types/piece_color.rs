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
}
