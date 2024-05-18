use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, Entity, EventReader, MouseButton, Query, ResMut, Transform, Vec3Swizzles, With,
};
use bevy::window::Window;

use crate::actions::{is_sprite_clicked_vec3, screen_pos_to_world_pos};
use crate::components::{Piece, Tile};
use crate::constants::{BALL_SIZE_SCALED, TILE_SIZE_SCALED};
use crate::resources::SelectionInfo;

pub fn select_piece(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_windows: Query<&Window>,
    q_camera: Query<&Transform, With<Camera>>,
    q_pieces: Query<(Entity, &Transform), With<Piece>>,
    q_tiles: Query<&Transform, With<Tile>>,
    mut selection_info: ResMut<SelectionInfo>,
) {
    let window = q_windows.single();
    let camera = q_camera.single();

    for event in mouse_button_input_events.read() {
        let world_pos = screen_pos_to_world_pos(window, camera);
        if world_pos.is_none() {
            continue;
        }
        let world_pos = world_pos.unwrap();

        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            let piece_clicked = q_pieces
                .iter()
                .find(|(entity, transform)| {
                    is_sprite_clicked_vec3(transform.translation, world_pos, BALL_SIZE_SCALED)
                })
                .map(|(entity, _)| entity);

            let tile_clicked = q_tiles.iter().find(|transform| {
                is_sprite_clicked_vec3(transform.translation, world_pos, TILE_SIZE_SCALED)
            });

            match (tile_clicked, piece_clicked, selection_info.selected()) {
                (Some(_), Some(piece), Some(selected_piece)) if piece == selected_piece => {
                    selection_info.deselect();
                    println!("Piece deselected: {}", piece.index());
                }
                (Some(_), Some(piece), _) => {
                    selection_info.select(piece);
                    println!("Piece selected: {}", piece.index());
                }
                (Some(tile_clicked), None, Some(_)) => {
                    selection_info.set_target(tile_clicked.translation.xy());
                    println!("Tile selected: {}", tile_clicked.translation.xy());
                }
                _ => {}
            }
        }
    }
}
