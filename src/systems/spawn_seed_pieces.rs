use crate::actions::tile_vec_to_world_pos;
use crate::constants::{BALL_LAYER, BALL_SCALE};
use crate::resources::Board;
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Res, Sprite, SpriteBundle, Transform};

pub fn spawn_seed_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    for piece in board.pieces.iter() {
        let position = tile_vec_to_world_pos(piece.tile_position());
        let mut color = piece.color();

        commands.spawn(
            (SpriteBundle {
                texture: asset_server.load("sprites/ball.png"),
                transform: Transform::default()
                    .with_translation(Vec3::new(position.x, position.y, BALL_LAYER))
                    .with_scale(Vec3::splat(BALL_SCALE)),
                sprite: Sprite { color, ..default() },
                ..default()
            }),
        );
    }
}
