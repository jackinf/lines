use bevy::prelude::{Entity, Resource, Vec2};

#[derive(Resource)]
pub struct SelectionInfo {
    entity: Option<Entity>,
    path: Vec<Vec2>,
}

impl SelectionInfo {
    pub fn new() -> Self {
        Self {
            entity: None,
            path: vec![],
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

    pub fn set_path(&mut self, path: Vec<Vec2>) {
        self.path = path;
    }

    pub fn get_path(&self) -> Vec<Vec2> {
        self.path.clone()
    }
}
