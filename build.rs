extern crate embed_resource;
use image::{imageops, GenericImageView, RgbaImage};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let source_path = Path::new("source_assets/");
    let dest_path = Path::new("assets/textures/");
    let file_names = ["grass_top.png", "dirt.png", "water.png"]; // Example filenames

    // Load the first image to get width and height
    let first_image_path = source_path.join(file_names[0]);
    let first_image = image::open(first_image_path).expect("Failed to load first image");
    let (width, height) = first_image.dimensions();
    let target_size = 16;

    let mut concatenated_image = RgbaImage::new(width, height * file_names.len() as u32);

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

    let dest_file = dest_path.join("tile_textures.png");
    fs::create_dir_all(dest_path).expect("Failed to create destination directory");
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
