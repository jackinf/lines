use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    AlignItems, Color, Commands, EventReader, JustifyContent, NodeBundle, PositionType, Res,
    ResMut, Style, TextBundle, TextStyle, Val,
};

use crate::events::ShowGameOverEvent;
use crate::resources::{Score, SelectionInfo};

pub fn show_game_over_event_hander(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    mut selection_info: ResMut<SelectionInfo>,
    mut show_game_over_event_reader: EventReader<ShowGameOverEvent>,
) {
    for _ in show_game_over_event_reader.read() {
        selection_info.set_game_over();

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.with_a(0.5).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Game Over!",
                        TextStyle {
                            font: asset_server.load("fonts/AmericanCaptain.ttf"),
                            font_size: 200.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Relative,
                        ..Default::default()
                    }),
                );

                // show score
                parent.spawn(
                    TextBundle::from_section(
                        &format!("Score: {}", score.0),
                        TextStyle {
                            font: asset_server.load("fonts/AmericanCaptain.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Relative,
                        top: Val::Px(200.0),
                        ..Default::default()
                    }),
                );
            });
    }
}
