use crate::components::ScoreText;
use bevy::asset::AssetServer;
use bevy::prelude::{
    default, Camera2dBundle, Commands, PositionType, Res, Style, TextBundle, TextStyle, Val,
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/AmericanCaptain.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
}
