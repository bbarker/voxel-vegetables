// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_voxel_world::prelude::*;
use speed_farmer::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VoxelWorldPlugin::default().with_voxel_texture("example_voxel_texture.png", 3)) // TODO: don't harcode 3
        .add_plugins(GamePlugin)
        .add_systems(Startup, set_window_icon);
    #[cfg(feature = "debug-inspector")]
    {
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = match windows.get_window(primary_entity) {
        Some(window) => window,
        None => {
            warn!("No primary window found");
            return;
        }
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
