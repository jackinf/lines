use bevy::prelude::{Entity, Resource, Vec2};

#[derive(Resource)]
pub struct SelectionInfo {
    entity: Option<Entity>,
    path: Vec<Vec2>,
    moving: bool,
}

impl SelectionInfo {
    pub fn new() -> Self {
        Self {
            entity: None,
            path: vec![],
            moving: false,
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

    pub fn pop_path(&mut self) {
        self.path.remove(0);
    }

    pub fn empty_path(&self) -> bool {
        self.path.is_empty()
    }

    pub fn start_moving(&mut self) {
        self.moving = true;
    }

    pub fn stop_moving(&mut self) {
        self.moving = false;
    }

    pub fn is_moving(&self) -> bool {
        self.moving
    }
}
