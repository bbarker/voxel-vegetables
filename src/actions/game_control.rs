use bevy::prelude::{Input, KeyCode, MouseButton, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Space,
    C,
    ClickTarget,
    OpenMenu,
}

impl GameControl {
    pub fn pressed(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        mouse_input: &Res<Input<MouseButton>>,
    ) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
            GameControl::Space => keyboard_input.pressed(KeyCode::Space),
            GameControl::C => keyboard_input.pressed(KeyCode::C),
            GameControl::ClickTarget => mouse_input.pressed(MouseButton::Left),
            GameControl::OpenMenu => keyboard_input.pressed(KeyCode::Escape),
        }
    }
}

pub fn get_movement(
    control: GameControl,
    key_input: &Res<Input<KeyCode>>,
    mouse_input: &Res<Input<MouseButton>>,
) -> f32 {
    if control.pressed(key_input, mouse_input) {
        1.0
    } else {
        0.0
    }
}
