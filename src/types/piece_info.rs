use bevy::math::Vec2;
use bevy::prelude::Color;

pub struct PieceInfo {
    tile_position: Vec2,
    color: Color,
}

impl PieceInfo {
    pub fn new(position: Vec2, color: Color) -> Self {
        Self {
            tile_position: position,
            color,
        }
    }

    pub fn tile_position(&self) -> Vec2 {
        self.tile_position
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
