use bevy::prelude::*;

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
#[derive(Clone, Debug, Component)]
pub struct GerminationTimer(pub f32); // Timer to track time for germination

pub struct GerminationNeeds {
    pub water: Water,
    pub soil: Soil,
}

/// Unlike GerminationNeeds, GrowingNeeds are cumulative
#[derive(PartialEq, Clone, Debug)]
pub struct GrowingNeeds {
    pub water: Water,
    pub soil: Soil,
    pub light: f32, // may want to chagne this
    pub time: f32,  // may want to change this
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
            (Species::Wheat, LifePhase::Seed) => BlockType::SeedPlanted,
            (Species::Wheat, LifePhase::Germinated) => BlockType::SeedPlanted,
            (Species::Wheat, LifePhase::Growing { .. }) => BlockType::WheatSprouts,
            (Species::Wheat, LifePhase::Mature) => BlockType::Wheat,
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
