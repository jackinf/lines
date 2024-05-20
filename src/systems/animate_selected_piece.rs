use bevy::prelude::{Entity, Query, ResMut, Transform, With};
use crate::components::Piece;
use crate::resources::SelectionInfo;

pub fn animate_selected_piece(
    q_pieces: Query<(&Transform, &Piece), With<Piece>>,
    selection_info: ResMut<SelectionInfo>,
) {
    if !selection_info.is_choosing() {
        return;
    }

    // selection_info.selected()
    //     .and_then(|id| q_pieces.iter().get(id))

}