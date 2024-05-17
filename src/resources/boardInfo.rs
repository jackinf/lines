use crate::types::PieceInfo;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Board {
    pub pieces: Vec<PieceInfo>,
}

impl Board {
    pub fn new() -> Self {
        Self { pieces: vec![] }
    }

    pub fn push(&mut self, piece: PieceInfo) {
        self.pieces.push(piece);
    }
}
