use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct Water(u32); // Representing quantity of water currently accessible

#[derive(Clone, Debug, Component)]
pub struct Soil(u32); // Representing quantity of soil currently accessible

#[derive(Clone, Debug, Component)]
pub struct GerminationTimer(f32); // Timer to track time for germination

pub struct GerminationNeeds {
    water: Water,
    soil: Soil,
    time: f32, // may want to change this
}
#[derive(Clone, Debug)]
enum Species {
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
}

#[derive(Clone, Debug, Component)]
pub struct Seed {
    species: Species,
}

#[derive(Clone, Debug, Component)]
pub struct Germinated {/* Data for germinated stage */}

#[derive(Clone, Debug, Component)]
pub struct Growing {/* Data for growing stage */}

#[derive(Clone, Debug, Component)]
pub struct Pollinated {/* Data for pollinated stage */}

#[derive(Clone, Debug, Component)]
pub struct Fruiting {/* Data for fruiting stage */}

pub fn seed_to_germinate_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Seed, &mut GerminationTimer, &Water, &Soil)>,
) {
    for (entity, mut seed, mut timer, water, soil) in query.iter_mut() {
        let needs = seed.species.germination_needs();

        timer.0 -= time.delta_seconds();

        if timer.0 <= 0.0 && water.0 >= needs.water.0 && soil.0 >= needs.soil.0 {
            // If conditions are met, transition from Seed to Germinated
            commands
                .entity(entity)
                .remove::<Seed>()
                .remove::<GerminationTimer>()
                .insert(Germinated {/* ... */});
        }
    }
}
