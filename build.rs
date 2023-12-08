extern crate embed_resource;

#[path = "src/build_common.rs"]
mod build_common;
use build_common::*;
use image::{imageops, RgbaImage};
use std::env;
use std::fs;

fn main() {
    let file_names = ["grass_top.png", "dirt.png", "water.png"]; // Example filenames

    // Load the first image to get width and height
    let source_path = asset_source_path();
    let target_size = 32;

    let mut concatenated_image = RgbaImage::new(target_size, target_size * file_names.len() as u32);

    file_names.iter().enumerate().for_each(|(ii, file_name)| {
        let image_path = source_path.join(file_name);
        let image = image::open(image_path)
            .expect("Failed to load image")
            .resize_exact(target_size, target_size, imageops::FilterType::Nearest);

        image::imageops::overlay(
            &mut concatenated_image,
            &image,
            0,
            ii as i64 * (target_size as i64),
        );
    });

    let dest_file = tile_texture_pack_path();
    fs::create_dir_all(asset_texture_path()).expect("Failed to create destination directory");
    concatenated_image
        .save(dest_file.clone())
        .expect("Failed to save concatenated image");

    println!(
        "Tile texture assembly complete: {}",
        dest_file.as_path().to_str().unwrap_or("")
    );

    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc");
    }
}
