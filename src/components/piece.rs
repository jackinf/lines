use crate::constants::Coord;
use crate::types::PieceColor;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Piece {
    coord: Coord,
    piece_color: PieceColor,
}

impl Piece {
    pub fn new(coord: Coord, piece_color: PieceColor) -> Self {
        Self { coord, piece_color }
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn set_coord(&mut self, coord: Coord) {
        self.coord = coord;
    }

    pub fn piece_color(&self) -> PieceColor {
        self.piece_color.clone()
    }
}
