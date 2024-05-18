use crate::constants::Coord;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Tile {
    coord: Coord,
}

impl Tile {
    pub fn new(coord: Coord) -> Self {
        Self { coord }
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }
}
