use std::collections::HashSet;

use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{Commands, default, EventReader, EventWriter, PositionType, Query, Res, ResMut, Sprite, SpriteBundle, Style, TextBundle, TextStyle, Transform, Val, Window};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::actions::tile_to_world_pos;
use crate::components::Piece;
use crate::constants::{BALL_LAYER, BALL_SCALE, Coord, MAX_PIECES};
use crate::events::spawn_new_pieces_event::SpawnNewPiecesEvent;
use crate::events::validate_move_event::{NextPlannedMove, ValidateMoveEvent};
use crate::resources::SelectionInfo;
use crate::types::PieceColor;

pub fn spawn_new_pieces_event_handler(
    mut validate_move_event_writer: EventWriter<ValidateMoveEvent>,
    mut spawn_new_pieces_event_reader: EventReader<SpawnNewPiecesEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_pieces: Query<&Piece>,
    q_window: Query<&Window>,
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

            // spawn game over text in the middle of the screen
            // get center of the screen
            let window = q_window.single();
            let center = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
            commands.spawn((
                TextBundle::from_section(
                    "Game Over!",
                    TextStyle {
                        font: asset_server.load("fonts/AmericanCaptain.ttf"),
                        font_size: 200.0,
                        ..default()
                    },
                )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(center.y - 100.),
                        left: Val::Px(center.x - 300.),
                        ..default()
                    }),
            ));

            return;
        }

        let pieces_to_create = create_seed_pieces(amount, &mut taken_pieces);

        for (id, (coord, piece_color)) in pieces_to_create.into_iter().enumerate() {
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
                Piece::new(id + 1, coord, piece_color),
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
