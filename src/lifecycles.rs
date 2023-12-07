use crate::core_components::*;
use crate::voxel_painting::paint_voxel_unchecked;
use crate::GameState;
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::collections::HashSet;

pub fn seed_to_germinate_system(
    time: Res<Time>,
    mut commands: Commands,
    mut voxel_world: VoxelWorld,
    mut query: Query<(
        Entity,
        &HasPosition,
        &Species,
        &LifePhase,
        &mut GerminationTimer,
        &Water,
        &Soil,
    )>,
) {
    query.for_each_mut(
        |(entity, HasPosition { pos }, species, life_phase, mut timer, water, soil)| {
            if *life_phase == LifePhase::Seed {
                let needs = species.germination_needs();

                timer.0 -= time.delta_seconds();

                if timer.0 <= 0.0 && water.0 >= needs.water.0 && soil.0 >= needs.soil.0 {
                    // If conditions are met, transition from Seed to Germinated
                    commands
                        .entity(entity)
                        .remove::<LifePhase>()
                        .remove::<GerminationTimer>()
                        // We just add a time delay to go from Germinated to Growing
                        .insert(GerminationTimer(0.0))
                        .insert(LifePhase::Germinated {/* ... */});
                    paint_voxel_unchecked(
                        &mut voxel_world,
                        *pos,
                        species.block_type(&LifePhase::Germinated),
                    );
                }
            }
        },
    )
}

/// We just add a time delay to go from Germinated to Growing
pub fn germinated_to_growing_system(
    time: Res<Time>,
    mut voxel_world: VoxelWorld,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &HasPosition,
        &Species,
        &LifePhase,
        &mut GerminationTimer,
    )>,
) {
    query.for_each_mut(
        |(entity, HasPosition { pos }, species, life_phase, mut timer)| {
            if *life_phase == LifePhase::Germinated {
                timer.0 -= time.delta_seconds();

                if timer.0 <= 0.0 {
                    let phase = LifePhase::Growing {
                        needs: species.growing_needs(),
                    };
                    paint_voxel_unchecked(&mut voxel_world, *pos, species.block_type(&phase));
                    commands.entity(entity).remove::<LifePhase>().insert(phase);
                }
            }
        },
    )
}

// TODO: add voxel painting logic and resource requirements
pub fn growing_to_mature_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifePhase, &Water, &Soil)>,
) {
    let light = 1.0; // TODO: adjust based on weather
    query.for_each_mut(|(entity, mut life_phase, water, soil)| {
        if let LifePhase::Growing { mut needs } = life_phase.clone() {
            // Check there are some non-zero conditions for growth
            if needs.time <= 0.0 && needs.light <= 0.0 && *needs.water == 0 && *needs.soil == 0 {
                commands
                    .entity(entity)
                    .remove::<LifePhase>()
                    .insert(LifePhase::Mature);
            } else if water.0 > 0 && soil.0 > 0 && light > 0.01 {
                // Get rate-limiting resource value
                // (for now we assume 1:1 usage between each)
                let growth_value = f32::min(f32::min(water.0 as f32, soil.0 as f32), light);
                needs.time -= time.delta_seconds();
                needs.water = Water(needs.water.saturating_sub(growth_value as u32));
                needs.soil = Soil(needs.soil.saturating_sub(growth_value as u32));
                *life_phase = LifePhase::Growing { needs };
            } else {
                // TODO: maybe add some health logic later where entities could lose health
                //     : if resources are not enough to maintain life
            }
        }
    })
}

// TODO: needs to actually relate to chunks and blocks
pub fn spawn_life_in_chunk(
    commands: &mut Commands,
    species: Species,
    phase: LifePhase,
    count: u16,
) {
    (1..=count).for_each(|_| {
        commands.spawn((species.clone(), phase.clone()));
    })
}

// TODO: needs to actually relate to chunks and blocks
fn init_life(mut commands: Commands) {
    let init_species: HashSet<Species> = vec![Species::Apple, Species::Wheat].into_iter().collect();
    init_species.into_iter().for_each(|species| {
        spawn_life_in_chunk(
            &mut commands,
            Species::Apple,
            LifePhase::Seed,
            species.wild_organisms_per_chunk(),
        )
    })
}

pub struct LifeCyclesPlugin;

impl Plugin for LifeCyclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            seed_to_germinate_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            germinated_to_growing_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            growing_to_mature_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(Startup, init_life);
    }
}

/*
// TODO: get random voxels in each chunk

use bevy::math::UVec3;
use rand::Rng;
use std::collections::HashSet;
use std::iter;

fn get_random_voxels(chunk: &Chunk, num_voxels: usize) -> Vec<WorldVoxel> {
    let chunk_volume = CHUNK_SIZE_U * CHUNK_SIZE_U * CHUNK_SIZE_U;
    assert!(num_voxels < chunk_volume as usize, "numVoxels must be less than the chunk volume");

    let mut rng = rand::thread_rng();
    let mut selected_positions = HashSet::new();

    iter::repeat_with(|| {
        UVec3::new(
            rng.gen_range(0..CHUNK_SIZE_U) as u32,
            rng.gen_range(0..CHUNK_SIZE_U) as u32,
            rng.gen_range(0..CHUNK_SIZE_U) as u32,
        )
    })
    .filter(|pos| selected_positions.insert(*pos))
    .take(num_voxels)
    .map(|position| chunk.chunk_data.get_voxel(position))
    .collect()
}

*/
