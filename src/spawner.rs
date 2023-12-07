// TODO: move spawning logic here
use crate::core_components::*;
use bevy::prelude::*;

#[allow(dead_code)] // TODO
pub fn spawn_organism(
    commands: &mut Commands,
    species: Species,
    phase: LifePhase,
    pos: IVec3,
    player_entity_opt: Option<Entity>,
) -> Entity {
    let new_entity_id = commands
        .spawn((
            species.clone(),
            phase.clone(),
            HasPosition { pos },
            Water(1),              // TODO: hardcoded for now, should be species-specific
            Soil(1),               // TODO: hardcoded for now, should be species-specific
            GerminationTimer(5.0), // TODO: hardcoded for now, should be species-specific
        ))
        .id();
    if let Some(player_entity) = player_entity_opt {
        commands.entity(new_entity_id).insert(OwnedBy {
            owner: player_entity,
        });
    }
    new_entity_id
}
