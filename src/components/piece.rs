use crate::constants::Coord;
use crate::types::PieceColor;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Piece {
    id: usize,
    coord: Coord,
    piece_color: PieceColor,
}

impl Piece {
    pub fn new(id: usize, coord: Coord, piece_color: PieceColor) -> Self {
        Self {
            id,
            coord,
            piece_color,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn piece_color(&self) -> PieceColor {
        self.piece_color.clone()
    }
}
