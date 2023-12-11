use crate::GameState;
use bevy::prelude::*;
use crate::core_components::*;

pub struct TimerPlugin;

#[derive(Clone, Debug, Component)]
pub struct GameTimer{
    pub time: f32,
    pub is_active: bool,
}

/// This Plugin adds a little timer
impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_timer.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct OpenLink(&'static str);

fn run_timer(
    mut query: Query<&mut GameTimer>,
    time: Res<Time>,
    mut commands: Commands,
    querry: Query<(Option<&ChangeState>, Option<&OpenLink>)>,
    mut next_state: ResMut<NextState<GameState>>,
    entity_query: Query<Entity, With<GameTimer>>
){
    let entity = entity_query.single();

    for mut timer in query.iter_mut(){
        if !(timer.is_active) {
            return;
        }
        if timer.time < 0. {
            for (change_state, open_link) in &querry {
                if let Some(_state) = change_state {
                    commands.entity(entity).despawn();
                    next_state.set(GameState::Menu);
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }

            timer.is_active = false;
        }
        timer.time -= time.delta_seconds();
    }
}