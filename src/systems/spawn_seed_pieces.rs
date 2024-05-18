use std::collections::HashSet;

use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    default, BuildChildren, Color, Commands, Res, Sprite, SpriteBundle, Transform,
};
use bevy_prototype_lyon::draw::Stroke;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::actions::{tile_to_world_pos, tile_vec_to_world_pos};
use crate::components::Piece;
use crate::constants::{Coord, BALL_LAYER, BALL_SCALE, TILE_SIZE};
use crate::types::PieceColor;

pub fn spawn_seed_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (id, (coord, piece_color)) in create_seed_pieces().into_iter().enumerate() {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/ball.png"),
                transform: Transform::default()
                    .with_translation(tile_to_world_pos(coord).extend(BALL_LAYER))
                    .with_scale(Vec3::splat(BALL_SCALE)),
                sprite: Sprite {
                    color: piece_color.get_color(),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::new(id + 1, coord, piece_color))
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

fn create_seed_pieces() -> Vec<(Coord, PieceColor)> {
    let mut rng = rand::thread_rng();
    let mut positions = HashSet::new();

    (0..5)
        .map(|_| {
            (
                generate_position(&mut rng, &mut positions),
                PieceColor::choose_piece_color(),
            )
        })
        .collect()
}

/// Function to generate a random position ensuring no overlap
fn generate_position(rng: &mut ThreadRng, positions: &mut HashSet<Coord>) -> Coord {
    loop {
        let x: usize = rng.gen_range(0..9);
        let y: usize = rng.gen_range(0..9);
        let position = (x, y);
        if positions.insert(position) {
            return position;
        }
    }
}
