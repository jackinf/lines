use crate::systems::{
    create_seed_pieces, move_pieces, select_piece, select_piece_destination, spawn_board,
    spawn_camera, spawn_score, spawn_seed_pieces,
};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, PreStartup, Startup, Update};
use bevy::DefaultPlugins;

mod actions;
mod components;
mod constants;
mod resources;
mod systems;
mod types;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(resources::Board::new())
        .insert_resource(resources::Score::new())
        .add_systems(
            PreStartup,
            (
                create_seed_pieces,
                spawn_camera,
                spawn_seed_pieces,
                spawn_score,
                spawn_board,
            )
                .chain(),
        )
        .add_systems(Update, (select_piece, select_piece_destination))
        .add_systems(FixedUpdate, (move_pieces))
        .run();
}
