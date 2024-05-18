use bevy::prelude::{Entity, Resource, Vec2};

#[derive(Resource)]
pub struct SelectionInfo {
    entity: Option<Entity>,
    target: Option<Vec2>,
}

impl SelectionInfo {
    pub fn new() -> Self {
        Self {
            entity: None,
            target: None,
        }
    }

    pub fn selected(&self) -> Option<Entity> {
        self.entity
    }

    pub fn select(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn deselect(&mut self) {
        self.entity = None;
    }

    pub fn is_selected(&self) -> bool {
        self.entity.is_some()
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.target = Some(target);
    }

    pub fn get_target(&self) -> Option<Vec2> {
        self.target
    }
}
