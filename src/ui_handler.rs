use std::fmt::Alignment;

use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

pub struct UiHandlerPlugin;

/// This plugin handles the ui related stuff like the displaying of entitys and the crosshair
/// is only active during the State `GameState::Playing`
impl Plugin for UiHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), render_ui);
    }
}

fn render_ui(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    windows: Query<&Window>,
) {
    let window: &Window = windows.single();
    
    // draw a crosshair onto the screen
    commands.spawn(ImageBundle {
        image: textures.crosshair.clone().into(),
        style: Style {
            align_self: AlignSelf::Center,
            left: Val::Px((window.width() / 2.) - 32.),
            ..default()
        },
        ..default()
    });

    // render the score, resources and the entities
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children
                .spawn(TextBundle::from_section(
                    "Score: 100",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            children
                .spawn(TextBundle::from_section(
                    "Resources: 100",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            children
                .spawn(TextBundle::from_section(
                    "Entities: 100",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
        });
}