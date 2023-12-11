use std::u8;

use bevy::utils::HashMap;
use lazy_static::*;
use strum_macros::*;
// BlockType maps to VoxTexture; see Block_TO_TILES_MAP below
#[derive(Clone, Copy, Debug, EnumIter, EnumCount, FromRepr)]
#[repr(u8)]
pub enum VoxTexture {
    // Refers to tile_textures.png; must be in the same order
    Nothing,
    Grass,
    Dirt,
    Water,
    GrassPlant,
    GrassTan,
    GrassBrown,
    WheatSide,
    WheatTop,
    WheatSprouts,
}

impl VoxTexture {
    pub fn index(self) -> u8 {
        self as u8
    }
    // fn from_unsafe(ix: u8) -> Self {
    //     VoxTexture::try_from(ix).unwrap()
    // }
}

/// Note the order is Top, Side, Bottom
#[derive(Clone, Copy, Debug)]
pub struct VoxTextureArray(pub [VoxTexture; 3]);

impl VoxTextureArray {
    // pub fn indices(self) -> [u8; 3] {
    //     self.0.map(|x| x.index())
    // }
    #[allow(dead_code)]
    pub fn indices_u32(self) -> [u32; 3] {
        self.0.map(|x| x.index() as u32)
    }
}

#[derive(Clone, Copy, Debug, EnumIter, EnumCount, FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BlockType {
    Grass,
    Dirt,
    Water,
    SeedPlanted,
    WheatSprouts,
    Wheat,
    AppleSapling,
    AppleTree,
    DeadTree,
    // AppleTreeWithApples,
    Nothing, // Keep Nothing as last in enum
}

impl BlockType {
    #[allow(dead_code)]
    pub fn index(self) -> u8 {
        self as u8
    }
    #[allow(dead_code)]
    pub fn from_unsafe(ix: u8) -> Self {
        BlockType::from_repr(ix).unwrap()
    }
}

lazy_static! {
    // TODO: use a macro that can iterate over enum values, then do a match
    //     : to make sure we cover all insertions.
    pub static ref BLOCK_TO_TILES_MAP: HashMap<BlockType, VoxTextureArray> = {
        let mut tmap = HashMap::new();
        tmap.insert(
            BlockType::Grass,
            VoxTextureArray([VoxTexture::Grass, VoxTexture::Grass, VoxTexture::Grass]),
        );
        tmap.insert(
            BlockType::Dirt,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::Water,
            VoxTextureArray([VoxTexture::Water, VoxTexture::Water, VoxTexture::Water]),
        );
        tmap.insert(
            BlockType::SeedPlanted,
            VoxTextureArray([VoxTexture::Nothing, VoxTexture::GrassPlant, VoxTexture::Nothing]),
        );
        tmap.insert(
            BlockType::WheatSprouts,
            VoxTextureArray([VoxTexture::WheatSprouts, VoxTexture::WheatSprouts, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::Wheat,
            VoxTextureArray([VoxTexture::WheatTop, VoxTexture::WheatSide, VoxTexture::Nothing]),
        );
        tmap.insert(
            BlockType::AppleSapling,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::AppleTree,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::DeadTree,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::Nothing,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap
    };
}

#[allow(dead_code)]
pub const GRASS_TEXTURE_ARRAY: VoxTextureArray =
    VoxTextureArray([VoxTexture::Grass, VoxTexture::Grass, VoxTexture::Grass]);

#[cfg(test)]
mod tests {
    use crate::block_types::*;

    #[test]
    fn grass_is_zero() {
        assert_eq!(BlockType::Grass.index(), 0)
    }
}
