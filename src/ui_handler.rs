use bevy::ecs::entity::Entities;
use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

pub struct UiHandlerPlugin;

/// This plugin handles the ui related stuff like the displaying of entitys and the crosshair
/// is only active during the State `GameState::Playing`
impl Plugin for UiHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HudData::default());
        app.add_systems(Update, render_ui.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Clone, Debug, Resource, Default)]
struct HudData {
    time_since_update: f32,
    entities: u32,
}

fn render_ui(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut hud_data: ResMut<HudData>,
    windows: Query<&Window>,
    entities: &Entities,
) {
    let window: &Window = windows.single();

    hud_data.time_since_update += time.delta_seconds();
    if hud_data.time_since_update > 1.0 {
        hud_data.time_since_update = 0.0;
        hud_data.entities = entities.len();
        info!(
            // TODO: remove once displaying properly in the UI
            "{}",
            ["Entities: ".to_string(), hud_data.entities.to_string()].join(" ")
        );
    }

    // render the score, resources and the entities
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                "Score: 100",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
            children.spawn(TextBundle::from_section(
                "Resources: 100",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
            children.spawn(TextBundle::from_section(
                ["Entities: ".to_string(), hud_data.entities.to_string()].join(" "),
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });

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
}
