use bevy::utils::HashMap;
use lazy_static::*;
// BlockType maps to VoxTexture; see Block_TO_TILES_MAP below
use num_enum::TryFromPrimitive;
#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum VoxTexture {
    // Refers to tile_textures.png
    Grass = 0,
    Dirt = 1,
    Water = 2,
}

impl VoxTexture {
    pub fn index(self) -> u8 {
        self as u8
    }
    // fn from_unsafe(ix: u8) -> Self {
    //     VoxTexture::try_from(ix).unwrap()
    // }
}

#[derive(Clone, Copy, Debug)]
pub struct VoxTextureArray(pub [VoxTexture; 3]);

impl VoxTextureArray {
    // pub fn indices(self) -> [u8; 3] {
    //     self.0.map(|x| x.index())
    // }
    pub fn indices_u32(self) -> [u32; 3] {
        self.0.map(|x| x.index() as u32)
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive, PartialEq, Eq, Hash)]
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
    pub fn index(self) -> u8 {
        self as u8
    }
    pub fn from_unsafe(ix: u8) -> Self {
        BlockType::try_from(ix).unwrap()
    }
}

lazy_static! {
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
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::WheatSprouts,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
        );
        tmap.insert(
            BlockType::Wheat,
            VoxTextureArray([VoxTexture::Dirt, VoxTexture::Dirt, VoxTexture::Dirt]),
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
