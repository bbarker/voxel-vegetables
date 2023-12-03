use bevy::prelude::*;

#[derive(Clone, Debug, Component, Deref)]
pub struct Water(u32); // Representing quantity of water currently accessible

#[derive(Clone, Debug, Component, Deref)]
pub struct Soil(u32); // Representing quantity of soil currently accessible

/// Used twice: for initial germination, and to go from germination to growing
#[derive(Clone, Debug, Component)]
pub struct GerminationTimer(f32); // Timer to track time for germination

pub struct GerminationNeeds {
    water: Water,
    soil: Soil,
    time: f32, // may want to change this
}

/// Unlike GerminationNeeds, GrowingNeeds are cumulative
#[derive(Clone, Debug)]
pub struct GrowingNeeds {
    water: Water,
    soil: Soil,
    light: f32, // may want to chagne this
    time: f32,  // may want to change this
}

#[derive(Clone, Debug, Component)]
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
                time: 1.0,
            },
            Species::Wheat => GerminationNeeds {
                water: Water(1),
                soil: Soil(1),
                time: 1.0,
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
}

#[derive(Clone, Debug, Component)]
pub struct Seed {}

#[derive(Clone, Debug, Component)]
pub struct Germinated {/* Data for germinated stage */}

#[derive(Clone, Debug, Component)]
pub struct Growing {
    needs: GrowingNeeds,
}

#[derive(Clone, Debug, Component)]
pub struct Mature {/* Mature-specific data */}

#[derive(Clone, Debug, Component)]
pub struct Pollinated {/* Data for pollinated stage */}

#[derive(Clone, Debug, Component)]
pub struct Fruiting {/* Data for fruiting stage */}

pub fn seed_to_germinate_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Species,
        &Seed,
        &mut GerminationTimer,
        &Water,
        &Soil,
    )>,
) {
    query.for_each_mut(|(entity, species, mut seed, mut timer, water, soil)| {
        let needs = species.germination_needs();

        timer.0 -= time.delta_seconds();

        if timer.0 <= 0.0 && water.0 >= needs.water.0 && soil.0 >= needs.soil.0 {
            // If conditions are met, transition from Seed to Germinated
            commands
                .entity(entity)
                .remove::<Seed>()
                .remove::<GerminationTimer>()
                // We just add a time delay to go from Germinated to Growing
                .insert(GerminationTimer(0.0))
                .insert(Germinated {/* ... */});
        }
    })
}

/// We just add a time delay to go from Germinated to Growing
pub fn germinated_to_growing_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Species,
        &Germinated,
        &mut GerminationTimer,
        &Water,
        &Soil,
    )>,
) {
    query.for_each_mut(|(entity, species, mut seed, mut timer, water, soil)| {
        timer.0 -= time.delta_seconds();

        if timer.0 <= 0.0 {
            commands.entity(entity).insert(Growing {
                needs: species.growing_needs(),
            });
        }
    })
}

pub fn growing_to_mature_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Species, &mut Growing, &Water, &Soil)>,
) {
    let light = 1.0; // TODO: adjust based on weather
    query.for_each_mut(|(entity, species, mut growing, water, soil)| {
        // Check there are some non-zero conditions for growth
        if growing.needs.time <= 0.0
            && growing.needs.light <= 0.0
            && *growing.needs.water == 0
            && *growing.needs.soil == 0
        {
            commands
                .entity(entity)
                .remove::<Growing>()
                .insert(Mature {/* ... */});
        } else if water.0 > 0 && soil.0 > 0 && light > 0.01 {
            // Get rate-limiting resource value
            // (for now we assume 1:1 usage between each)
            let growth_value = f32::min(f32::min(water.0 as f32, soil.0 as f32), light);
            growing.needs.time -= time.delta_seconds();
            growing.needs.water = Water(growing.needs.water.saturating_sub(growth_value as u32));
            growing.needs.soil = Soil(growing.needs.soil.saturating_sub(growth_value as u32));
        } else {
            // TODO: maybe add some health logic later where entities could lose health
            // if resources are not enough to maintain life
        }
    })
}

pub struct LifeCyclesPlugin;

impl Plugin for LifeCyclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, seed_to_germinate_system)
            .add_systems(Update, germinated_to_growing_system)
            .add_systems(Update, growing_to_mature_system);
    }
}
