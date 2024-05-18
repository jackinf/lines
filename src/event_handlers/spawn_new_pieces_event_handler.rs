use std::collections::HashSet;

use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    default, BuildChildren, Color, Commands, EventReader, EventWriter, Query, Res, ResMut, Sprite,
    SpriteBundle, Transform,
};
use bevy_prototype_lyon::draw::Stroke;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::actions::tile_to_world_pos;
use crate::components::Piece;
use crate::constants::{Coord, BALL_LAYER, BALL_SCALE, MAX_PIECES, TILE_SIZE};
use crate::events::spawn_new_pieces_event::SpawnNewPiecesEvent;
use crate::events::validate_move_event::{ValidateMoveEvent, ValidationType};
use crate::resources::SelectionInfo;
use crate::types::PieceColor;

pub fn spawn_new_pieces_event_handler(
    mut validate_move_event_writer: EventWriter<ValidateMoveEvent>,
    mut spawn_new_pieces_event_reader: EventReader<SpawnNewPiecesEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_pieces: Query<&Piece>,
    mut selection_info: ResMut<SelectionInfo>,
) {
    for spawn_new_pieces_event in spawn_new_pieces_event_reader.read() {
        let mut taken_pieces = q_pieces
            .iter()
            .map(|piece| piece.coord())
            .collect::<HashSet<Coord>>();
        let amount = spawn_new_pieces_event.amount();
        let diff = MAX_PIECES - taken_pieces.iter().count();
        let amount = amount.min(diff);

        if amount == 0 {
            selection_info.set_game_over();
            // TODO: publish game over event
            return;
        }

        let pieces_to_create = create_seed_pieces(amount, &mut taken_pieces);

        for (id, (coord, piece_color)) in pieces_to_create.into_iter().enumerate() {
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

        validate_move_event_writer.send(ValidateMoveEvent::new(ValidationType::PostSpawn));
    }
}

fn create_seed_pieces(
    amount: usize,
    mut taken_pieces: &mut HashSet<Coord>,
) -> Vec<(Coord, PieceColor)> {
    let mut rng = rand::thread_rng();

    (0..amount)
        .map(|_| {
            (
                generate_position(&mut rng, &mut taken_pieces),
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
