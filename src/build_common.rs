use std::path::{Path, PathBuf};

pub const ASSET_SOURCE_DIR: &str = "source_assets/";
#[allow(dead_code)]
pub fn asset_source_path() -> PathBuf {
    Path::new(ASSET_SOURCE_DIR).to_path_buf()
}

#[allow(dead_code)]
pub const ASSET_TEXTURE_REL_DIR: &str = "textures/";

pub const ASSET_TEXTURE_DIR: &str = "assets/textures/";
pub fn asset_texture_path() -> PathBuf {
    Path::new(ASSET_TEXTURE_DIR).to_path_buf()
}

pub const TILE_TEXTURE_PACK_FILE: &str = "tile_textures.png";
#[allow(dead_code)]
pub fn tile_texture_pack_file() -> PathBuf {
    Path::new(TILE_TEXTURE_PACK_FILE).to_path_buf()
}

#[allow(dead_code)]
pub fn tile_texture_pack_path() -> PathBuf {
    asset_texture_path().join(TILE_TEXTURE_PACK_FILE)
}

#[allow(dead_code)]
pub fn tile_texture_pack_rel_path() -> PathBuf {
    Path::new(ASSET_TEXTURE_REL_DIR)
        .to_path_buf()
        .join(TILE_TEXTURE_PACK_FILE)
}
