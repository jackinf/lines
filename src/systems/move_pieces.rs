use crate::components::Piece;
use crate::constants::BALL_SPEED;
use crate::resources::SelectionInfo;
use bevy::prelude::{Query, Res, ResMut, Time, Transform, Vec3, With};

pub fn move_pieces(
    time: Res<Time>,
    mut q_pieces: Query<&mut Transform, With<Piece>>,
    mut selection_info: ResMut<SelectionInfo>,
) {
    if !selection_info.is_moving() {
        return;
    }

    if let Some(selected_piece) = selection_info.selected() {
        if let Some(mut piece_transform) = q_pieces.get_mut(selected_piece).ok() {
            if let Some(&next_destination) = selection_info.get_path().first() {
                println!("=====================================");
                println!("Current position: {:?}", piece_transform.translation);
                println!("Moving towards {:?}", next_destination);
                let dt = time.delta_seconds();
                let direction =
                    (next_destination.extend(0.0) - piece_transform.translation).normalize();
                println!("Direction: {:?}", direction);
                let new_translation = piece_transform.translation + direction * BALL_SPEED * dt;
                piece_transform.translation = Vec3::new(
                    new_translation.x,
                    new_translation.y,
                    piece_transform.translation.z,
                );
                println!("New translation: {:?}", piece_transform.translation);

                let distance = piece_transform
                    .translation
                    .truncate()
                    .distance(next_destination);
                println!("Distance: {}", distance);
                if distance < 0.1 {
                    println!("POP! Left to move: {}", selection_info.get_path().len() - 1);
                    selection_info.pop_path();
                    if selection_info.empty_path() {
                        selection_info.stop_moving();
                        selection_info.deselect();
                    }
                }
            }
        }
    }

    // let path = selection_info.get_path();
    // if path.is_empty() {
    //     return;
    // }
    // let next_destination = path.first().unwrap();
    //
    // if let Some(selected_piece) = selection_info.selected() {
    //     if let Some(mut piece_transform) = q_pieces.get_mut(selected_piece) {
    //         let dt = time.delta_seconds();
    //
    //         // move towards the next destination
    //         let direction = piece_transform.translation - next_destination.extend(0.0);
    //         let distance = direction.length();
    //         let direction = direction.normalize();
    //         let speed = BALL_SPEED;
    //
    //         let new_translation = piece_transform.translation - direction * speed * dt;
    //         let new_translation = Vec3::new(new_translation.x, new_translation.y, piece_transform.translation.z);
    //         piece_transform.translation = new_translation;
    //
    //         if distance < 0.1 {
    //             selection_info.pop_path();
    //         }
    //     }
    // }
}
