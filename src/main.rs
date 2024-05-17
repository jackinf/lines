use crate::systems::{draw_board, redraw_pieces, select_destination, select_piece, setup};
use bevy::prelude::{App, FixedUpdate, Startup, Update};
use bevy::DefaultPlugins;

mod actions;
mod components;
mod constants;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_board))
        .add_systems(Update, (select_piece, select_destination))
        .add_systems(FixedUpdate, (redraw_pieces))
        .run();
}
