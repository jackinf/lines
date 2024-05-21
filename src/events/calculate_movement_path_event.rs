use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct CalculateMovementPathEvent {
    piece_id: Entity,
    target_tile_id: Entity,
}

impl CalculateMovementPathEvent {
    pub fn new(piece_id: Entity, target_tile_id: Entity) -> Self {
        Self {
            piece_id,
            target_tile_id,
        }
    }

    pub fn piece_id(&self) -> Entity {
        self.piece_id
    }

    pub fn target_tile_id(&self) -> Entity {
        self.target_tile_id
    }
}
