use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, PreStartup, Update};
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::systems::{
    move_pieces, select_piece, spawn_board, spawn_camera, spawn_score, spawn_seed_pieces,
};

mod actions;
mod components;
mod constants;
mod resources;
mod systems;
mod types;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        // .insert_resource(resources::Board::new())
        .insert_resource(resources::Score::new())
        .insert_resource(resources::SelectionInfo::new())
        .add_systems(
            PreStartup,
            (spawn_camera, spawn_seed_pieces, spawn_score, spawn_board).chain(),
        )
        .add_systems(Update, select_piece)
        .add_systems(FixedUpdate, (move_pieces))
        .run();
}
