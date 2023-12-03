use bevy::prelude::*;
use bevy::window::WindowMode;
use speed_farmer::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run()
}
