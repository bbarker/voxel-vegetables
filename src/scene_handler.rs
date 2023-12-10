use crate::GameState;
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

pub struct SceneSwitchPlugin;

/// This Plugin configurats the cameras so that only one is active at a time
impl Plugin for SceneSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras)
            .add_systems(OnEnter(GameState::Playing), enter_playing)
            .add_systems(OnEnter(GameState::Menu), enter_menu);
    }
}

#[derive(Component)]
pub struct MenuCamera;

fn setup_cameras(
    mut commands: Commands,
){
    // Spawn Cameras

    // Menu Camera
    commands.spawn((Camera2dBundle::default(), MenuCamera));

    // Game Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-200.0, 180.0, -200.0),
            ..default()
        },
        // This tells bevy_voxel_world tos use this cameras transform to calculate spawning area
        VoxelWorldCamera,
    ));
}

fn enter_playing(
    mut menu_camera_query: Query<&mut Camera, With<MenuCamera>>,
    mut game_camera_query: Query<&mut Camera, (With<VoxelWorldCamera>, Without<MenuCamera>)>,
){
    let mut menu_camera = menu_camera_query.single_mut();
    menu_camera.is_active = false;

    let mut game_camera = game_camera_query.single_mut();
    game_camera.is_active = true;
}

fn enter_menu(
    mut menu_camera_query: Query<&mut Camera, With<MenuCamera>>,
    mut game_camera_query: Query<&mut Camera, (With<VoxelWorldCamera>, Without<MenuCamera>)>,
){
    let mut menu_camera = menu_camera_query.single_mut();
    menu_camera.is_active = true;

    let mut game_camera = game_camera_query.single_mut();
    game_camera.is_active = false;
}