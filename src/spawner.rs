// TODO: move spawning logic here
use crate::core_components::*;
use bevy::prelude::*;

#[allow(dead_code)] // TODO
pub fn spawn_organism(
    commands: &mut Commands,
    species: Species,
    phase: LifePhase,
    pos: IVec3,
) -> Entity {
    commands
        .spawn((species.clone(), phase.clone(), HasPosition { pos }))
        .id()
}
