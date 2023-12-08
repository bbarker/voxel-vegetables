use crate::actions::Actions;
use crate::core_components::*;
use crate::voxel_painting::get_surface_air_voxel;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_voxel_world::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_click.run_if(in_state(GameState::Playing)))
            .add_systems(Update, open_menu)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut cam_transform: Query<&mut Transform, (With<VoxelWorldCamera>, Without<Player>)>,
) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 200., 1.)),
                ..Default::default()
            },
            PlayerInventory::new(),
        ))
        .insert(Player);

    cam_transform.single_mut().translation = Vec3::new(0., 200., 1.);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut cam_query: Query<&mut Transform, (With<VoxelWorldCamera>, Without<Player>)>,
) {
    if let Some(action_player_movement) = actions.player_movement {
        let speed = 150.0;
        let movement_speed =
            action_player_movement.normalize_or_zero() * speed * time.delta_seconds();
        let cam_transform = cam_query.single();
        let movement = if movement_speed.x.is_normal() {
            movement_speed.x * cam_transform.right().normalize_or_zero()
        } else if movement_speed.y.is_normal() {
            movement_speed.y * cam_transform.up().normalize_or_zero()
        } else if movement_speed.z.is_normal() {
            movement_speed.z * cam_transform.forward().normalize_or_zero()
        } else {
            Vec3::ZERO
        };
        player_query.for_each_mut(|mut player_transform| {
            player_transform.translation += movement;
            cam_query.single_mut().translation += movement;
        })
    }
}

fn player_click(
    mut commands: Commands,
    mut voxel_world: VoxelWorld,
    actions: Res<Actions>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    cam_query: Query<&Transform, (With<VoxelWorldCamera>, Without<Player>)>,
) {
    if actions.left_click_crosshair {
        if let Ok(cam_transform) = cam_query.get_single() {
            let click_direction = cam_transform.forward().normalize_or_zero();
            player_query.for_each(|(player_entity, player_transform)| {
                if let Some(voxel_pos) = get_surface_air_voxel(
                    &mut voxel_world,
                    player_transform.translation,
                    click_direction,
                ) {
                    let _managed_id = commands.spawn((PlayerWantsToPaintVoxel {
                        player: player_entity,
                        pos: voxel_pos,
                        paint_as: PaintableResources::SeedCrop(Species::Wheat),
                    },));
                    debug!("player painted voxel at {}", voxel_pos);
                } else {
                    debug!("nothing to paint in direction {}", click_direction)
                }
            })
        }
    }
}

#[derive(Component)]
struct OpenLink(&'static str);

// FIXME: doesn't seem to do anything
fn open_menu(
    mut commands: Commands,
    actions: Res<Actions>,
    querry: Query<(Option<&ChangeState>, Option<&OpenLink>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (change_state, open_link) in &querry {
        if actions.open_menu {
            if let Some(state) = change_state {
                next_state.set(state.0.clone());
            } else if let Some(link) = open_link {
                if let Err(error) = webbrowser::open(link.0) {
                    warn!("Failed to open link {error:?}");
                }
            }
        }
    }
    if actions.open_menu {
        commands.spawn(ChangeState(GameState::Menu));
    }
}

fn cleanup(mut game_camera_query: Query<&mut Camera, With<VoxelWorldCamera>>, mut windows: Query<&mut Window>) {
    let mut camera = game_camera_query.single_mut();
    camera.is_active = false;

    let mut window = windows.single_mut();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
