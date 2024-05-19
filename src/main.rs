use crate::event_handlers::{
    show_game_over_event_hander, spawn_new_pieces_event_handler, validate_move_event_handler,
};
use crate::events::{ShowGameOverEvent, SpawnNewPiecesEvent, ValidateMoveEvent};
use crate::systems::{move_pieces, select_piece, spawn_board, spawn_camera, spawn_score, start};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, PreStartup, Update};
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::ShapePlugin;

mod actions;
mod components;
mod constants;
mod event_handlers;
mod events;
mod resources;
mod systems;
mod types;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_event::<ValidateMoveEvent>()
        .add_event::<SpawnNewPiecesEvent>()
        .add_event::<ShowGameOverEvent>()
        .add_systems(
            Update,
            (
                validate_move_event_handler,
                spawn_new_pieces_event_handler,
                show_game_over_event_hander,
            ),
        )
        .insert_resource(resources::Score::new())
        .insert_resource(resources::SelectionInfo::new())
        .add_systems(
            PreStartup,
            (start, spawn_camera, spawn_score, spawn_board).chain(),
        )
        .add_systems(Update, select_piece)
        .add_systems(FixedUpdate, move_pieces)
        .run();
}
