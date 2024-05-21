use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, Entity, EventReader, EventWriter, MouseButton, Query, ResMut, Transform, With,
};
use bevy::window::Window;

use crate::actions::{is_sprite_clicked_vec3, screen_pos_to_world_pos};
use crate::components::{Piece, Tile};
use crate::constants::{BALL_SIZE_SCALED, TILE_SIZE_SCALED};
use crate::events::{CalculateMovementPathEvent, CenterPieceToTileEvent};
use crate::resources::SelectionInfo;

pub fn select_piece(
    q_windows: Query<&Window>,
    q_camera: Query<&Transform, With<Camera>>,
    q_pieces: Query<(Entity, &Transform, &Piece), With<Piece>>,
    q_tiles: Query<(Entity, &Transform), With<Tile>>,
    mut selection_info: ResMut<SelectionInfo>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut center_piece_to_tile_event_writer: EventWriter<CenterPieceToTileEvent>,
    mut calculate_movement_path_event_writer: EventWriter<CalculateMovementPathEvent>,
) {
    if !selection_info.is_choosing() {
        return;
    }

    let window = q_windows.single();
    let camera = q_camera.single();

    for event in mouse_button_input_events.read() {
        let world_pos = screen_pos_to_world_pos(window, camera);
        if world_pos.is_none() {
            continue;
        }
        let world_pos = world_pos.unwrap();

        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            let clicked_piece = q_pieces
                .iter()
                .find(|(_, transform, _)| {
                    is_sprite_clicked_vec3(transform.translation, world_pos, BALL_SIZE_SCALED)
                })
                .map(|(entity, _, _)| entity);

            let tile_clicked = q_tiles.iter().find(|(_, transform)| {
                is_sprite_clicked_vec3(transform.translation, world_pos, TILE_SIZE_SCALED)
            });
            if tile_clicked.is_none() {
                continue;
            }
            let (tile_id, _) = tile_clicked.unwrap();

            match (clicked_piece, selection_info.selected()) {
                (Some(clicked_piece_id), Some(selected_piece_id))
                    if clicked_piece_id == selected_piece_id =>
                {
                    let piece_id = selected_piece_id;
                    selection_info.deselect();
                    center_piece_to_tile_event_writer
                        .send(CenterPieceToTileEvent::new(tile_id, piece_id));
                }
                (Some(clicked_piece_id), _) => {
                    selection_info.select(clicked_piece_id);
                }
                (None, Some(selected_piece_id)) => {
                    calculate_movement_path_event_writer
                        .send(CalculateMovementPathEvent::new(selected_piece_id, tile_id));
                }
                _ => {}
            }
        }
    }
}
