use std::collections::HashSet;

use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    default, Commands, EventReader, EventWriter, Query, Res, Sprite, SpriteBundle, Transform,
};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::actions::tile_to_world_pos;
use crate::components::Piece;
use crate::constants::{Coord, BALL_LAYER, BALL_SCALE, MAX_PIECES};
use crate::events::{NextPlannedMove, ShowGameOverEvent, SpawnNewPiecesEvent, ValidateMoveEvent};
use crate::types::PieceColor;

pub fn spawn_new_pieces_event_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_pieces: Query<&Piece>,
    mut validate_move_event_writer: EventWriter<ValidateMoveEvent>,
    mut spawn_new_pieces_event_reader: EventReader<SpawnNewPiecesEvent>,
    mut show_game_over_event_writer: EventWriter<ShowGameOverEvent>,
) {
    for spawn_new_pieces_event in spawn_new_pieces_event_reader.read() {
        let mut taken_pieces = q_pieces
            .iter()
            .map(|piece| piece.coord())
            .collect::<HashSet<Coord>>();
        let amount = spawn_new_pieces_event.amount();
        let diff = MAX_PIECES - taken_pieces.iter().count();
        let amount = amount.min(diff);

        if diff == 0 {
            show_game_over_event_writer.send(ShowGameOverEvent);
            return;
        }

        let pieces_to_create = create_seed_pieces(amount, &mut taken_pieces);

        for (coord, piece_color) in pieces_to_create.into_iter() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/ball.png"),
                    transform: Transform::default()
                        .with_translation(tile_to_world_pos(coord).extend(BALL_LAYER))
                        .with_scale(Vec3::splat(BALL_SCALE)),
                    sprite: Sprite {
                        color: piece_color.get_color(),
                        ..default()
                    },
                    ..default()
                },
                Piece::new(coord, piece_color.clone()),
            ));
        }

        validate_move_event_writer.send(ValidateMoveEvent::new(NextPlannedMove::Play));
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
