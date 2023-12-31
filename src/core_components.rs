use bevy::{prelude::*, utils::HashMap};

use crate::{block_types::BlockType, GameState};

#[derive(Component)]
pub struct ChangeState(pub GameState);

// we have to be careful about multiple sources of truth
#[derive(Eq, PartialEq, Clone, Debug, Component)]
pub struct HasPosition {
    pub pos: IVec3,
}

#[derive(Eq, PartialEq, Clone, Debug, Component)]
pub struct OwnedBy {
    pub owner: Entity,
}

#[derive(Eq, PartialEq, Clone, Debug, Component, Deref)]
pub struct Water(pub u32); // Representing quantity of water currently accessible

#[derive(Eq, PartialEq, Clone, Debug, Component, Deref)]
pub struct Soil(pub u32); // Representing quantity of soil currently accessible

/// Used twice: for initial germination, and to go from germination to growing
#[derive(Clone, Debug, Component, Deref)]
pub struct GerminationTimer(pub f32); // Timer to track time for germination

pub struct GerminationNeeds {
    pub water: Water,
    pub soil: Soil,
}

#[derive(Clone, Debug, Component)]
pub struct MatureAgeTimer(pub f32);

/// Unlike GerminationNeeds, GrowingNeeds are cumulative
#[derive(PartialEq, Clone, Debug)]
pub struct GrowingNeeds {
    pub water: Water,
    pub soil: Soil,
    pub light: f32,
    pub time: f32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct FruitingNeeds {
    pub water: Water,
    pub soil: Soil,
    pub light: f32,
    pub time: f32,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Component)]
pub enum Species {
    Apple,
    Wheat,
}

impl Species {
    pub fn germination_needs(&self) -> GerminationNeeds {
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

    pub fn growing_needs(&self) -> GrowingNeeds {
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

    pub fn fruiting_needs(&self) -> FruitingNeeds {
        match self {
            Species::Apple => FruitingNeeds {
                water: Water(20),
                soil: Soil(20),
                light: 40.0,
                time: 40.0,
            },
            Species::Wheat => FruitingNeeds {
                water: Water(3),
                soil: Soil(3),
                light: 10.0,
                time: 30.0,
            },
        }
    }

    pub fn seeds_per_generation(&self) -> u32 {
        match self {
            Species::Apple => 15,  // Actual per tree is about 1500
            Species::Wheat => 100, // 30412800 - actual estimate per chunk if we had real wheat
        }
    }

    /// should be roughly proportional to seeds_per_generation
    pub fn spread_per_fruiting(&self) -> u8 {
        match self {
            Species::Apple => 2,
            Species::Wheat => 10,
        }
    }

    pub fn spread_distance(&self) -> u8 {
        match self {
            Species::Apple => 10,
            Species::Wheat => 50,
        }
    }

    pub fn food_value(&self) -> u32 {
        match self {
            Species::Apple => 10,
            Species::Wheat => 2,
        }
    }

    pub fn wild_organisms_per_chunk(&self) -> u16 {
        match self {
            Species::Apple => 1,
            Species::Wheat => 10,
        }
    }

    pub fn block_type(&self, phase: &LifePhase) -> BlockType {
        match (self, phase) {
            (Species::Apple, LifePhase::Seed) => BlockType::SeedPlanted,
            (Species::Apple, LifePhase::Germinated) => BlockType::SeedPlanted,
            (Species::Apple, LifePhase::Growing { .. }) => BlockType::AppleSapling,
            (Species::Apple, LifePhase::Mature) => BlockType::AppleTree,
            (Species::Apple, LifePhase::Pollinated { .. }) => BlockType::AppleTree,
            (Species::Apple, LifePhase::Fruiting) => BlockType::AppleTree,
            (Species::Apple, LifePhase::Death) => BlockType::DeadTree,
            (Species::Wheat, LifePhase::Seed) => BlockType::SeedPlanted,
            (Species::Wheat, LifePhase::Germinated) => BlockType::SeedPlanted,
            (Species::Wheat, LifePhase::Growing { .. }) => BlockType::WheatSprouts,
            (Species::Wheat, LifePhase::Mature) => BlockType::Wheat,
            (Species::Wheat, LifePhase::Pollinated { .. }) => BlockType::Wheat,
            (Species::Wheat, LifePhase::Fruiting) => BlockType::Wheat,
            (Species::Wheat, LifePhase::Death) => BlockType::Nothing,
        }
    }

    // pub fn self_pollinates(&self) -> bool {
    //     match self {
    //         Species::Apple => false,
    //         Species::Wheat => true,
    //     }
    // }

    pub fn min_generations(&self) -> u32 {
        match self {
            Species::Apple => 10,
            Species::Wheat => 1,
        }
    }

    pub fn max_generations(&self) -> u32 {
        match self {
            Species::Apple => 200,
            Species::Wheat => 1,
        }
    }

    pub fn decay_time(&self) -> f32 {
        match self {
            Species::Apple => 200.,
            Species::Wheat => 0.,
        }
    }
}

//TODO: in general, when there are a lot of entities of a given
// component, it probably makes sense to not use an enum for the
// component itself, but instead for the individual variants,
// This way the ECS can likely optimize the query
#[derive(PartialEq, Clone, Debug, Component)]
pub enum LifePhase {
    Seed,
    Germinated,
    Growing { needs: GrowingNeeds },
    Mature,
    Pollinated { needs: FruitingNeeds },
    Fruiting,
    Death,
}

#[derive(PartialEq, Clone, Debug, Component, Deref)]
pub struct Generations(pub u32);

#[derive(Clone, Debug, Component, Deref)]
pub struct DecayTimer(pub f32);

pub const SEED_PHASE: LifePhase = LifePhase::Seed;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PaintableResources {
    SeedCrop(Species),
}

#[derive(Eq, PartialEq, Clone, Debug, Component)]
pub struct PlayerWantsToPaintVoxel {
    pub player: Entity,
    pub pos: IVec3,
    pub paint_as: PaintableResources,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum FarmResource {
    FoodValue(Species),
    Seeds(Species),
}

pub fn add_resources(
    res_map1: HashMap<FarmResource, u32>,
    res_map2: HashMap<FarmResource, u32>,
) -> HashMap<FarmResource, u32> {
    res_map1
        .into_iter()
        .chain(res_map2)
        .fold(HashMap::new(), |mut acc_map, (res, amount)| {
            *acc_map.entry(res).or_insert(0) += amount;
            acc_map
        })
}

#[derive(Eq, PartialEq, Clone, Debug, Component)]
pub struct PlayerInventory {
    pub resources: HashMap<FarmResource, u32>,
}

impl PlayerInventory {
    pub fn new() -> Self {
        PlayerInventory {
            resources: HashMap::new(),
        }
    }

    // pub fn add_resources(mut self, res_map: HashMap<FarmResource, u32>) {
    //     self.resources = add_resources(self.resources, res_map);
    // }

    pub fn add_resources(&mut self, res_map: HashMap<FarmResource, u32>) {
        self.resources = add_resources(self.resources.clone(), res_map);
    }
}

// TODO: it was mentioned using Events might be an alternative to this
#[derive(Eq, PartialEq, Clone, Debug, Component)]
pub struct CollectResource {
    pub owner: Entity,
    pub resource: HashMap<FarmResource, u32>,
}

impl CollectResource {
    pub fn new(owner: Entity) -> Self {
        CollectResource {
            owner,
            resource: HashMap::new(),
        }
    }

    /// Keeps owner of this collect, if other's is different
    pub fn add(self, other: CollectResource) -> Self {
        CollectResource {
            owner: self.owner,
            resource: add_resources(self.resource, other.resource),
        }
    }
    // pub fn add(self, other: HashMap<FarmResource, u32>) -> Self {
    //     CollectResource {
    //         owner: self.owner,
    //         resource: add_resources(self.resource, other),
    //     }
    // }
}
