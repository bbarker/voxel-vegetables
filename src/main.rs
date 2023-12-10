// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod block_types;
mod build_common;

use crate::block_types::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_voxel_world::prelude::*;
use build_common::*;
use std::io::Cursor;
use strum::EnumCount;
use voxel_vegetables::GamePlugin;
use winit::window::Icon;

fn main() {
    let tile_texture_file_path = tile_texture_pack_rel_path();
    let tile_texture_file = tile_texture_file_path
        .to_str()
        .unwrap_or_else(|| panic!("Couldn't find texture file '{:?}'!", tile_texture_file_path));
    info!("tile texture file is {}", tile_texture_file);
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Voxel Vegetables".to_string(),
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
        .add_plugins(
            VoxelWorldPlugin::default()
                .with_voxel_texture(tile_texture_file, VoxTexture::COUNT as u32),
        )
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
