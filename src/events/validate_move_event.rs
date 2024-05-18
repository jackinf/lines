use bevy::prelude::Event;

#[derive(PartialEq, Clone)]
pub enum ValidationType {
    PreSpawn,
    PostSpawn,
}

// TODO: add enum for pre-validation and post-validation
#[derive(Event)]
pub struct ValidateMoveEvent {
    validation_type: ValidationType,
}

impl ValidateMoveEvent {
    pub fn new(validation_type: ValidationType) -> Self {
        Self { validation_type }
    }

    pub fn validation_type(&self) -> ValidationType {
        self.validation_type.clone()
    }
}
