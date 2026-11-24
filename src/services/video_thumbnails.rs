// src/services/video_thumbnails.rs

use anyhow::{bail, Result};
use std::path::Path;
use std::process::Command;

use crate::services::storage_service;

pub fn generate_video_thumbnail(
    video_path: &str,
) -> Result<String> {
    println!(
        "Génération miniature vidéo : {}",
        video_path
    );

    let source =
        Path::new(video_path);

    if !source.exists() {
        bail!(
            "Vidéo introuvable : {}",
            video_path
        );
    }

    let thumbnail_path =
        storage_service::video_thumbnail_path(
            video_path
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

    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            video_path,
            "-ss",
            "00:00:01",
            "-vframes",
            "1",
        ])
        .arg(&thumbnail_path)
        .status()?;

    if !status.success() {

        bail!(
            "FFmpeg a échoué pour : {}",
            video_path
        );
    }

    println!(
        "Miniature vidéo enregistrée : {}",
        thumbnail_path.display()
    );

    Ok(
        thumbnail_path
            .to_string_lossy()
            .to_string()
    )
}