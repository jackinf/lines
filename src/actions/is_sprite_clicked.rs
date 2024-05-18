use bevy::math::Vec2;
use bevy::prelude::Vec3;

pub fn is_sprite_clicked_vec3(translation: Vec3, cursor_world_pos: Vec2, sprite_size: f32) -> bool {
    is_sprite_clicked_vec2(translation.truncate(), cursor_world_pos, sprite_size)
}

pub fn is_sprite_clicked_vec2(translation: Vec2, cursor_world_pos: Vec2, sprite_size: f32) -> bool {
    let sprite_size = Vec2::new(sprite_size, sprite_size);

    let min = translation - sprite_size / 2.0;
    let max = translation + sprite_size / 2.0;

    let in_x = (min.x..max.x).contains(&cursor_world_pos.x);
    let in_y = (min.y..max.y).contains(&-cursor_world_pos.y);

    in_x && in_y
}
