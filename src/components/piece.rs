use bevy::math::Vec2;
use bevy::prelude::{Color, Component};

#[derive(Component)]
pub struct Piece {
    position: Vec2,
    color: Color,
}

impl Piece {
    pub fn new(position: Vec2, color: Color) -> Self {
        Self { position, color }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
