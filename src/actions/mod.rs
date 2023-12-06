use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

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
    pub player_movement: Option<Vec3>,
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
    let player_movement = Vec3::new(
        get_movement(GameControl::Right, &keyboard_input, &mouse_input)
            - get_movement(GameControl::Left, &keyboard_input, &mouse_input),
        get_movement(GameControl::Space, &keyboard_input, &mouse_input)
            - get_movement(GameControl::C, &keyboard_input, &mouse_input),
        get_movement(GameControl::Up, &keyboard_input, &mouse_input)
            - get_movement(GameControl::Down, &keyboard_input, &mouse_input),
    );

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

    if player_movement != Vec3::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }

    actions.left_click_crosshair = GameControl::ClickTarget.pressed(&keyboard_input, &mouse_input);
    actions.open_menu = GameControl::OpenMenu.pressed(&keyboard_input, &mouse_input);
}
