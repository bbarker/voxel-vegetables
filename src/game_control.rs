use bevy::prelude::{KeyCode, MouseButton};
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum MovementControl {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum GameControl {
    Movement(MovementControl),
    ClickTarget,
    OpenMenu,
}

impl GameControl {
    pub fn from_key_code(key_code: KeyCode) -> Option<GameControl> {
        match key_code {
            KeyCode::W | KeyCode::Up => Some(GameControl::Movement(MovementControl::Forward)),
            KeyCode::S | KeyCode::Down => Some(GameControl::Movement(MovementControl::Backward)),
            KeyCode::A | KeyCode::Left => Some(GameControl::Movement(MovementControl::Left)),
            KeyCode::D | KeyCode::Right => Some(GameControl::Movement(MovementControl::Right)),
            KeyCode::Space => Some(GameControl::Movement(MovementControl::Up)),
            KeyCode::C => Some(GameControl::Movement(MovementControl::Down)),
            KeyCode::Escape => Some(GameControl::OpenMenu),
            _ => None,
        }
    }

    pub fn movement_control(game_control: GameControl) -> Option<MovementControl> {
        if let GameControl::Movement(movement_control) = game_control {
            Some(movement_control)
        } else {
            None
        }
    }

    pub fn from_mouse_button(mouse_button: MouseButton) -> Option<GameControl> {
        match mouse_button {
            MouseButton::Left => Some(GameControl::ClickTarget),
            _ => None,
        }
    }
}
