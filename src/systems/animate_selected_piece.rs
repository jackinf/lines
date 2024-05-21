use bevy::prelude::{Query, Res, ResMut, Time, Transform, With};

use crate::components::Piece;
use crate::resources::SelectionInfo;

pub fn animate_selected_piece(
    time: Res<Time>,
    mut q_pieces: Query<(&mut Transform, &mut Piece), With<Piece>>,
    selection_info: ResMut<SelectionInfo>,
) {
    if !selection_info.is_choosing() {
        return;
    }

    if let Some(selected_piece) = selection_info.selected() {
        if let Ok((mut transform, mut piece)) = q_pieces.get_mut(selected_piece) {
            let mut bouncer = piece.bouncer();
            bouncer.add_step(1000. * time.delta_seconds());
            let delta = bouncer.get_y_delta();
            transform.translation.y += delta * time.delta_seconds();
        }
    }
}
