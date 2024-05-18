use bevy::asset::AssetServer;
use bevy::prelude::{default, Commands, Res, SpriteBundle, Transform, Vec3};

use crate::actions::tile_to_world_pos;
use crate::components::Tile;
use crate::constants::{BOARD_LAYER, TILES_HORIZONTAL, TILES_VERTICAL, TILE_SCALE};

pub fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    for row in 0..TILES_HORIZONTAL {
        for col in 0..TILES_VERTICAL {
            let coord = (row, col);
            let pos = tile_to_world_pos(coord);

            let square = if (row + col) % 2 == 0 {
                "sprites/square_light.png"
            } else {
                "sprites/square_dark.png"
            };

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(square),
                    transform: Transform::default()
                        .with_translation(pos.extend(BOARD_LAYER))
                        .with_scale(Vec3::splat(TILE_SCALE)),
                    ..default()
                },
                Tile::new(coord),
            ));
        }
    }
}
