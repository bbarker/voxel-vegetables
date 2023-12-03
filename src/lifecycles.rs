use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug, Component, Deref)]
pub struct Water(u32); // Representing quantity of water currently accessible

#[derive(Eq, PartialEq, Clone, Debug, Component, Deref)]
pub struct Soil(u32); // Representing quantity of soil currently accessible

/// Used twice: for initial germination, and to go from germination to growing
#[derive(Clone, Debug, Component)]
pub struct GerminationTimer(f32); // Timer to track time for germination

pub struct GerminationNeeds {
    water: Water,
    soil: Soil,
}

/// Unlike GerminationNeeds, GrowingNeeds are cumulative
#[derive(PartialEq, Clone, Debug)]
pub struct GrowingNeeds {
    water: Water,
    soil: Soil,
    light: f32, // may want to chagne this
    time: f32,  // may want to change this
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Component)]
pub enum Species {
    Apple,
    Wheat,
}

impl Species {
    fn germination_needs(&self) -> GerminationNeeds {
        match self {
            Species::Apple => GerminationNeeds {
                water: Water(1),
                soil: Soil(1),
            },
            Species::Wheat => GerminationNeeds {
                water: Water(1),
                soil: Soil(1),
            },
        }
    }

    fn growing_needs(&self) -> GrowingNeeds {
        match self {
            Species::Apple => GrowingNeeds {
                water: Water(50),
                soil: Soil(50),
                light: 100.0,
                time: 100.0,
            },
            Species::Wheat => GrowingNeeds {
                water: Water(10),
                soil: Soil(10),
                light: 10.0,
                time: 10.0,
            },
        }
    }

    pub fn wild_organisms_per_chunk(&self) -> u16 {
        match self {
            Species::Apple => 1,
            Species::Wheat => 10,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Component)]
pub enum LifePhase {
    Seed,
    Germinated,
    Growing { needs: GrowingNeeds },
    Mature,
    // Pollinated,
    // Fruiting,
}

pub fn seed_to_germinate_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Species,
        &LifePhase,
        &mut GerminationTimer,
        &Water,
        &Soil,
    )>,
) {
    query.for_each_mut(|(entity, species, life_phase, mut timer, water, soil)| {
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
            }
        }
    })
}

/// We just add a time delay to go from Germinated to Growing
pub fn germinated_to_growing_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Species, &LifePhase, &mut GerminationTimer)>,
) {
    query.for_each_mut(|(entity, species, life_phase, mut timer)| {
        if *life_phase == LifePhase::Germinated {
            timer.0 -= time.delta_seconds();

            if timer.0 <= 0.0 {
                commands.entity(entity).insert(LifePhase::Growing {
                    needs: species.growing_needs(),
                });
            }
        }
    })
}

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
        app.add_systems(Update, seed_to_germinate_system)
            .add_systems(Update, germinated_to_growing_system)
            .add_systems(Update, growing_to_mature_system)
            .add_systems(Startup, init_life);
    }
}
