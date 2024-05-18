use crate::components::Piece;
use crate::constants::BALL_SPEED;
use crate::events::validate_move_event::{ValidateMoveEvent, ValidationType};
use crate::resources::SelectionInfo;
use bevy::prelude::{EventWriter, Query, Res, ResMut, Time, Transform, Vec3, With};

pub fn move_pieces(
    time: Res<Time>,
    mut q_pieces: Query<(&mut Transform, &mut Piece), With<Piece>>,
    mut selection_info: ResMut<SelectionInfo>,
    mut validate_move_event_writer: EventWriter<ValidateMoveEvent>,
) {
    if !selection_info.is_moving() {
        return;
    }

    if let Some(selected_piece) = selection_info.selected() {
        if let Some((mut transform, mut piece)) = q_pieces.get_mut(selected_piece).ok() {
            if let Some(&next_destination) = selection_info.get_path().first() {
                let dt = time.delta_seconds();
                let direction = (next_destination.extend(0.0) - transform.translation).normalize();
                let new_translation = transform.translation + direction * BALL_SPEED * dt;
                transform.translation = Vec3::new(
                    new_translation.x,
                    new_translation.y,
                    transform.translation.z,
                );

                let distance = transform.translation.truncate().distance(next_destination);
                if distance < 0.1 {
                    selection_info.pop_path();
                    if selection_info.empty_path() {
                        if let Some(coord) = selection_info.pop_dest_coord() {
                            piece.set_coord(coord);
                        } else {
                            println!("THIS SHOULD NOT HAVE HAPPENED");
                        }

                        selection_info.deselect();
                        selection_info.validate_move();
                        validate_move_event_writer
                            .send(ValidateMoveEvent::new(ValidationType::PreSpawn));
                    }
                }
            }
        }
    }
}
