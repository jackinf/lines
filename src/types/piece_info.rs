use crate::types::piece_color::PieceColor;
use bevy::math::Vec2;

pub struct PieceInfo {
    tile_position: Vec2,
    piece_color: PieceColor,
}

impl PieceInfo {
    pub fn new(position: Vec2, piece_color: PieceColor) -> Self {
        Self {
            tile_position: position,
            piece_color,
        }
    }

    pub fn tile_position(&self) -> Vec2 {
        self.tile_position
    }

    pub fn piece_color(&self) -> PieceColor {
        self.piece_color.clone()
    }
}
