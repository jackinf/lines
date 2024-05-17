use crate::actions::tile_vec_to_world_pos::tile_vec_to_world_pos;
use bevy::prelude::Vec2;

pub fn tile_to_world_pos(tile: (i32, i32)) -> Vec2 {
    let (col, row) = tile;

    return tile_vec_to_world_pos(Vec2::new(col as f32, row as f32));
}
