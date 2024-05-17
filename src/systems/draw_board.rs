use crate::constants::{BOARD_OFFSET, TILE_SCALE, TILE_SIZE};
use bevy::asset::AssetServer;
use bevy::prelude::{default, Commands, Res, SpriteBundle, Transform, Vec3};

pub fn draw_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    for row in 0..9 {
        for col in 0..9 {
            let x = col as f32 * TILE_SIZE * TILE_SCALE - BOARD_OFFSET;
            let y = row as f32 * TILE_SIZE * TILE_SCALE - BOARD_OFFSET;

            let square = if (row + col) % 2 == 0 {
                "sprites/square_light.png"
            } else {
                "sprites/square_dark.png"
            };

            commands.spawn((SpriteBundle {
                texture: asset_server.load(square),
                transform: Transform::default()
                    .with_translation(Vec3::new(x, y, 0.0))
                    .with_scale(Vec3::splat(TILE_SCALE)),
                ..default()
            },));
        }
    }
}
