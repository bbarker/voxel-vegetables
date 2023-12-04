use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_voxel_world::prelude::*;

use crate::{loading::TextureAssets, GameState};

pub struct CameraHandlerPlugin;

/// This plugin makes the camera move
/// Camera logic is only active during the State `GameState::Playing`
impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_mouse)
            .add_systems(Update, move_camera.run_if(in_state(GameState::Playing)));
    }
}

fn move_camera(
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>,
    mut cam_transform: Query<&mut Transform, With<VoxelWorldCamera>>,
) {
    motion_evr.read().for_each(|ev| {
        let rotation_speed = 0.1; // Adjust this value as needed

        // Calculate the new rotation based on mouse input
        let delta_x = -(ev.delta.x * time.delta_seconds() * rotation_speed);
        let delta_y = -(ev.delta.y * time.delta_seconds() * rotation_speed);

        // Get the current rotation of the camera
        let mut camera_transform = cam_transform.single_mut();
        let current_rotation = camera_transform.rotation;

        // Create new rotations for x and y axes
        let rotation_x = Quat::from_rotation_x(delta_y);
        let rotation_y = Quat::from_rotation_y(delta_x);

        // Combine the new rotations with the current rotation
        let new_rotation = current_rotation * rotation_x * rotation_y;

        // Update the camera's rotation
        camera_transform.rotation = new_rotation;
    })
}

fn setup_mouse(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    commands.spawn(ImageBundle {
        image: textures.crosshair.clone().into(),
        style: Style {
            align_self: AlignSelf::Center,
            left: Val::Percent(50.),
            ..default()
        },
        ..default()
    });
}
