use bevy::prelude::*;

use crate::game_control::{GameControl, MovementControl};
use crate::GameState;

//pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_playing_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Vec<MovementControl>,
    pub left_click_crosshair: bool,
    pub open_menu: bool,
}

pub fn set_playing_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    //touch_input: Res<Touches>,
    //player: Query<&Transform, With<Player>>,
    //camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let game_controls_from_keys: Vec<GameControl> = keyboard_input
        .get_pressed()
        .filter_map(|key_code| GameControl::from_key_code(*key_code))
        .collect();
    let game_controls_from_mouse: Vec<GameControl> = mouse_input
        .get_pressed()
        .filter_map(|mouse_button| GameControl::from_mouse_button(*mouse_button))
        .collect();
    actions.player_movement = game_controls_from_keys
        .iter()
        .filter_map(|key_code| GameControl::movement_control(key_code.clone()))
        .collect();

    /*if let Some(touch_position) = touch_input.first_pressed_position() {
        let (camera, camera_transform) = camera.single();
        if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
        {
            let diff = touch_position - player.single().translation.xy();
            if diff.length() > FOLLOW_EPSILON {
                player_movement = diff.normalize();
            }
        }
    }*/
    actions.left_click_crosshair = game_controls_from_mouse
        .iter()
        .any(|x| *x == GameControl::ClickTarget);

    actions.open_menu = game_controls_from_keys
        .iter()
        .any(|x| *x == GameControl::OpenMenu);
}
