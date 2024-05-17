use crate::components::Piece;
use bevy::prelude::{Query, Transform, With};

pub fn move_pieces(q_pieces: Query<&Transform, With<Piece>>) {}
