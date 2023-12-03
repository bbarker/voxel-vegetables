use bevy::{prelude::*, a11y::accesskit::Action, input::mouse::MouseMotion};
use bevy_voxel_world::prelude::*;

use crate::GameState;

pub struct CameraHandlerPlugin;

/// This plugin makes the camera move
/// Camera logic is only active during the State `GameState::Playing`
impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera.run_if(in_state(GameState::Playing)));
    }
}

fn move_camera(
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>,
    mut cam_transform: Query<&mut Transform, With<VoxelWorldCamera>>,
) {
    for ev in motion_evr.iter() {
        cam_transform.single_mut().rotate_y(-(ev.delta.x * time.delta_seconds()));
        cam_transform.single_mut().rotate_x(-(ev.delta.y * time.delta_seconds()))
    }
}