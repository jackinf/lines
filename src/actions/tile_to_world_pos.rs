use crate::actions::tile_vec_to_world_pos::tile_vec_to_world_pos;
use crate::constants::Coord;
use bevy::prelude::Vec2;

pub fn tile_to_world_pos(tile: Coord) -> Vec2 {
    let (col, row) = tile;

    tile_vec_to_world_pos(Vec2::new(col as f32, row as f32))
}
