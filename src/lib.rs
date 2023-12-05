#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod camera_handler;
mod core_components;
mod lifecycles;
mod loading;
mod map_setup;
mod menu;
mod player;
mod spawner;
mod voxel_painting;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::camera_handler::CameraHandlerPlugin;
use crate::lifecycles::LifeCyclesPlugin;
use crate::loading::LoadingPlugin;
use crate::map_setup::map_setup;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            LifeCyclesPlugin,
            CameraHandlerPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
                .add_systems(Startup, map_setup);
        }
    }
}
