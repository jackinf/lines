use crate::constants::Coord;
use crate::types::PieceColor;
use bevy::prelude::Component;

pub struct Bouncer {
    step: f32,
    y_offset: f32,
}

impl Bouncer {
    pub fn new() -> Self {
        Self {
            y_offset: 0.0,
            step: 100.0,
        }
    }

    pub fn add_step(&mut self, y_offset: f32) {
        self.y_offset += y_offset;
    }

    pub fn get_y_delta(&self) -> f32 {
        let cycle_position = self.y_offset % 500.0;

        if cycle_position >= 0.0 && cycle_position < 250.0 {
            self.step
        } else {
            -self.step
        }
    }

    pub fn reset(&mut self) {
        self.y_offset = 0.0;
    }
}

#[derive(Component)]
pub struct Piece {
    coord: Coord,
    piece_color: PieceColor,
    bouncer: Bouncer,
}

impl Piece {
    pub fn new(coord: Coord, piece_color: PieceColor) -> Self {
        Self {
            coord,
            piece_color,
            bouncer: Bouncer::new(),
        }
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

    pub fn bouncer(&mut self) -> &mut Bouncer {
        &mut self.bouncer
    }
}
