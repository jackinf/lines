use bevy::prelude::Component;

#[derive(Component)]
pub struct Tile {
    row: i32,
    col: i32,
}

impl Tile {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }
}
