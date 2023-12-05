use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut cam_transform: Query<&mut Transform, (With<VoxelWorldCamera>, Without<Player>)>,
) {
    commands
        .spawn(SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 200., 1.)),
            ..Default::default()
        })
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
        for mut player_transform in &mut player_query {
            player_transform.translation += movement;
            cam_query.single_mut().translation += movement;
        }
    }
}
