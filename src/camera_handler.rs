use crate::actions::*;
use crate::game_control::*;
use crate::player::Player;
use crate::GameState;
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_voxel_world::prelude::*;
pub struct CameraHandlerPlugin;

/// This plugin makes the camera move
/// Camera logic is only active during the State `GameState::Playing`
impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_mouse)
            .add_systems(
                Update,
                mouse_motion_system.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                camera_movement_system.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct FlyCamera {
    /// The speed the FlyCamera accelerates at. Defaults to `1.5`
    pub accel: f32,
    /// The maximum speed the FlyCamera can move at. Defaults to `0.5`
    pub max_speed: f32,
    /// The sensitivity of the FlyCamera's motion based on mouse movement. Defaults to `3.0`
    pub sensitivity: f32,
    /// The amount of deceleration to apply to the camera's motion. Defaults to `1.0`
    pub friction: f32,
    /// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
    pub pitch: f32,
    /// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
    pub yaw: f32,
    /// The current velocity of the FlyCamera. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
    pub velocity: Vec3,
    pub enabled: bool,
}
impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            accel: 1.5,
            max_speed: 0.5,
            sensitivity: 3.0,
            friction: 1.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            enabled: true,
        }
    }
}

fn setup_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = forward_vector(rotation);

    Vec3::new(f.x, 0.0, f.z).normalize()
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
    // Rotate it 90 degrees to get the strafe direction
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}

fn camera_movement_system(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut query: Query<(&mut FlyCamera, &mut Transform), (With<VoxelWorldCamera>, Without<Player>)>,
) {
    for (mut options, mut transform) in query.iter_mut() {
        let (axis_h, axis_v, axis_float) = if options.enabled {
            (
                movement_axis(&actions, MovementControl::Right, MovementControl::Left),
                movement_axis(
                    &actions,
                    MovementControl::Backward,
                    MovementControl::Forward,
                ),
                movement_axis(&actions, MovementControl::Up, MovementControl::Down),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        let rotation = transform.rotation;
        let accel: Vec3 = (strafe_vector(&rotation) * axis_h)
            + (forward_walk_vector(&rotation) * axis_v)
            + (Vec3::Y * axis_float);
        let accel: Vec3 = if accel.length() != 0.0 {
            accel.normalize() * options.accel
        } else {
            Vec3::ZERO
        };

        let friction: Vec3 = if options.velocity.length() != 0.0 {
            options.velocity.normalize() * -1.0 * options.friction
        } else {
            Vec3::ZERO
        };

        options.velocity += accel * time.delta_seconds();

        // clamp within max speed
        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        options.velocity =
            if (options.velocity + delta_friction).signum() != options.velocity.signum() {
                Vec3::ZERO
            } else {
                options.velocity + delta_friction
            };

        transform.translation += options.velocity;
        if let Ok(mut player_transform) = player_query.get_single_mut() {
            player_transform.translation += options.velocity;
        }
    }
}

fn mouse_motion_system(
    time: Res<Time>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.read() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        if !options.enabled {
            continue;
        }
        options.yaw -= delta.x * options.sensitivity * time.delta_seconds();
        options.pitch += delta.y * options.sensitivity * time.delta_seconds();

        options.pitch = options.pitch.clamp(-89.0, 89.9);
        // println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
            * Quat::from_axis_angle(-Vec3::X, pitch_radians);
    }
}

pub fn movement_axis(actions: &Res<Actions>, plus: MovementControl, minus: MovementControl) -> f32 {
    let mut axis = 0.0;
    if actions.player_movement.contains(&plus) {
        axis += 1.0;
    }
    if actions.player_movement.contains(&minus) {
        axis -= 1.0;
    }
    axis
}
