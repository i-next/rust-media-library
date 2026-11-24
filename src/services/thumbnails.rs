use anyhow::Result;
use image::imageops::FilterType;
use std::path::Path;

use crate::services::storage_service;

pub fn generate_thumbnail(
    image_path: &str,
) -> Result<String> {

    let source =
        Path::new(image_path);

    let thumbnail_path =
        storage_service::image_thumbnail_path(
            image_path
    );
    storage_service::ensure_parent(
        &thumbnail_path
    )?;

    if thumbnail_path.exists() {

        return Ok(
            thumbnail_path
                .to_string_lossy()
                .to_string()
        );
    }

    let img =
        image::open(source)?;

    let thumb =
        img.resize(
            256,
            256,
            FilterType::Lanczos3
        );

    thumb.save(
        &thumbnail_path
    )?;

    println!(
        "Miniature enregistrée : {}",
        thumbnail_path.display()
    );

    Ok(
        thumbnail_path
            .to_string_lossy()
            .to_string()
    )
}
