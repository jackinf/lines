use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct CenterPieceToTileEvent {
    tile_id: Entity,
    piece_id: Entity,
}

impl CenterPieceToTileEvent {
    pub fn new(tile_id: Entity, piece_id: Entity) -> Self {
        Self { tile_id, piece_id }
    }

    pub fn tile_id(&self) -> Entity {
        self.tile_id
    }

    pub fn piece_id(&self) -> Entity {
        self.piece_id
    }
}
