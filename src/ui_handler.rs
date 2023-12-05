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
){
    let window: &Window = windows.single();
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