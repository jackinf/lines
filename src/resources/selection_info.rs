use crate::constants::Coord;
use bevy::prelude::{Entity, Resource, Vec2};
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum GameState {
    ChoosingPiece,
    MovingPiece,
    ValidatingMove,
    GameOver,
}

#[derive(Resource)]
pub struct SelectionInfo {
    entity: Option<Entity>,
    path: Vec<Vec2>,
    state: GameState,
    dest_coord: Option<Coord>,
}

impl SelectionInfo {
    pub fn new() -> Self {
        Self {
            entity: None,
            path: vec![],
            state: GameState::ChoosingPiece,
            dest_coord: None,
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

    pub fn start_choosing(&mut self) {
        self.state = GameState::ChoosingPiece;
    }

    pub fn is_choosing(&self) -> bool {
        self.state == GameState::ChoosingPiece
    }

    pub fn start_moving(&mut self) {
        self.state = GameState::MovingPiece;
    }

    pub fn is_moving(&self) -> bool {
        self.state == GameState::MovingPiece
    }

    pub fn validate_move(&mut self) {
        self.state = GameState::ValidatingMove;
    }

    pub fn is_validating(&self) -> bool {
        self.state == GameState::ValidatingMove
    }

    pub fn set_game_over(&mut self) {
        self.state = GameState::GameOver;
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }

    pub fn set_dest_coord(&mut self, coord: Coord) {
        self.dest_coord = Some(coord);
    }

    pub fn pop_dest_coord(&mut self) -> Option<Coord> {
        if self.dest_coord.is_none() {
            return None;
        }
        self.dest_coord.take()
    }
}
