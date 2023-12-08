#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    Grass,
    Dirt,
    // Water,
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
    /* // TODO: maybe use a HashMap in a OnceCell
    fn from_index(ix: u8) -> Option<Self> {
        if ix <= BlockType::Nothing.index() {
            Some(ix as BlockType)
        } else {
            None
        }
    }
    */
}

#[cfg(test)]
mod tests {
    use crate::block_types::*;

    #[test]
    fn grass_is_zero() {
        assert_eq!(BlockType::Grass.index(), 0)
    }
}
