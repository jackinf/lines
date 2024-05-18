use crate::types::PieceColor;
use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Piece {
    id: usize,
    tile_position: Vec2,
    piece_color: PieceColor,
}

impl Piece {
    pub fn new(id: usize, position: Vec2, color: PieceColor) -> Self {
        Self {
            id,
            tile_position: position,
            piece_color: color,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn tile_position(&self) -> Vec2 {
        self.tile_position
    }

    pub fn piece_color(&self) -> PieceColor {
        self.piece_color.clone()
    }
}
