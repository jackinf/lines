use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Score(pub i32);

impl Score {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&mut self, amount: i32) {
        self.0 += amount;
    }
}
