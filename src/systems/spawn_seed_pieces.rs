use crate::actions::tile_vec_to_world_pos;
use crate::components::Piece;
use crate::constants::{BALL_LAYER, BALL_SCALE, TILE_SCALE, TILE_SIZE};
use crate::resources::Board;
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    default, BuildChildren, Color, Commands, PositionType, Res, Sprite, SpriteBundle, Style,
    TextBundle, TextStyle, Transform, Val,
};
use bevy_prototype_lyon::draw::Stroke;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;

pub fn spawn_seed_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    for (id, piece) in board.pieces.iter().enumerate() {
        let tile_position = piece.tile_position();
        let world_position = tile_vec_to_world_pos(tile_position);
        let piece_color = piece.piece_color();
        let mut color = piece_color.get_color();

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/ball.png"),
                transform: Transform::default()
                    .with_translation(Vec3::new(world_position.x, world_position.y, BALL_LAYER))
                    .with_scale(Vec3::splat(BALL_SCALE)),
                sprite: Sprite { color, ..default() },
                ..default()
            })
            .insert(Piece::new(id + 1, tile_position, piece_color))
            .with_children(|parent| {
                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Circle {
                            radius: TILE_SIZE * BALL_SCALE,
                            ..default()
                        }),
                        ..default()
                    },
                    Stroke::new(Color::BLACK, 2.0),
                ));
            });
    }
}
