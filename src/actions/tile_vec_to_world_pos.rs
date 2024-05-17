use crate::constants::{BOARD_OFFSET, TILE_SCALE, TILE_SIZE};
use bevy::math::Vec2;

pub fn tile_vec_to_world_pos(tile: Vec2) -> Vec2 {
    let x = tile.x * TILE_SIZE * TILE_SCALE - BOARD_OFFSET;
    let y = tile.y * TILE_SIZE * TILE_SCALE - BOARD_OFFSET;

    Vec2::new(x, y)
}
