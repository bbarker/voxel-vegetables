use bevy::ecs::entity::Entities;
use bevy::prelude::*;

use crate::player::Player;
use crate::{core_components::PlayerInventory, loading::TextureAssets, GameState};
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
    resource_count: u32,
    score: u32,
}

#[derive(Component)]
struct Hud {}

#[allow(clippy::too_many_arguments)]
fn render_ui(
    mut commands: Commands,
    entities: &Entities,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut hud_data: ResMut<HudData>,
    windows: Query<&Window>,
    hud_query: Query<Entity, With<Hud>>,
    inventory_query: Query<&PlayerInventory, With<Player>>,
) {
    let window: &Window = windows.single();

    hud_data.time_since_update += time.delta_seconds();
    if hud_data.time_since_update > 1.0 {
        hud_data.time_since_update = 0.0;
        hud_data.entities = entities.len();
        hud_data.score = inventory_query
            .iter()
            .map(|inv| inv.resources.values().sum::<u32>())
            .sum();
        hud_data.resource_count = inventory_query
            .iter()
            .map(|inv| inv.resources.values().sum::<u32>())
            .sum();
    }

    hud_query.for_each(|hud| commands.entity(hud).despawn_recursive());
    // render the score, resources and the entities
    commands
        .spawn((
            (NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },),
            Hud {},
        ))
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                format!("Score: {}", hud_data.score),
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
            children.spawn(TextBundle::from_section(
                format!("Resources: {}", hud_data.resource_count),
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
    commands.spawn((
        (ImageBundle {
            image: textures.crosshair.clone().into(),
            style: Style {
                align_self: AlignSelf::Center,
                left: Val::Px((window.width() / 2.) - 32.),
                ..default()
            },
            ..default()
        },),
        Hud {},
    ));
}
