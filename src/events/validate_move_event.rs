use bevy::prelude::Event;

#[derive(PartialEq, Clone)]
pub enum NextPlannedMove {
    SpawnPieces,
    Play,
}

// TODO: add enum for pre-validation and post-validation
#[derive(Event)]
pub struct ValidateMoveEvent {
    next_planned_move: NextPlannedMove,
}

impl ValidateMoveEvent {
    pub fn new(next_planned_move: NextPlannedMove) -> Self {
        Self { next_planned_move }
    }

    pub fn next_planned_move(&self) -> NextPlannedMove {
        self.next_planned_move.clone()
    }
}
