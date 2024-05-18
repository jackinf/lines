use bevy::prelude::Event;

#[derive(Event)]
pub struct SpawnNewPiecesEvent {
    amount: usize,
}

impl SpawnNewPiecesEvent {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }

    pub fn amount(&self) -> usize {
        self.amount
    }
}
