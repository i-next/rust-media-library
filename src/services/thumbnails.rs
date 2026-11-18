use anyhow::Result;
use image::imageops::FilterType;
use std::fs;
use std::path::Path;

pub fn generate_thumbnail(image_path: &str) -> Result<String> {
    fs::create_dir_all("storage/thumbnails")?;

    let source = Path::new(image_path);

    let filename = source.file_stem().unwrap_or_default().to_string_lossy();

    let thumbnail_path = format!("storage/thumbnails/{}.jpg", filename);

    if Path::new(&thumbnail_path).exists() {
        return Ok(thumbnail_path);
    }

    let img = image::open(source)?;

    let thumb = img.resize(256, 256, FilterType::Lanczos3);

    thumb.save(&thumbnail_path)?;
    println!("Miniature enregistrée : {}", thumbnail_path);
    Ok(thumbnail_path)
}
