use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn generate_video_thumbnail(video_path: &str) -> Result<String> {
    fs::create_dir_all("storage/thumbnails")?;

    let source = Path::new(video_path);

    let filename = source
        .file_stem()
        .ok_or_else(|| anyhow!("Nom de fichier invalide"))?
        .to_string_lossy();

    let thumbnail_path = format!("storage/thumbnails/{}.jpg", filename);

    if Path::new(&thumbnail_path).exists() {
        return Ok(thumbnail_path);
    }

    let output = Command::new("ffmpeg")
        .args(["-y", "-ss", "00:00:03", "-i", video_path, "-frames:v", "1"])
        .arg(&thumbnail_path)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("ffmpeg a échoué"));
    }

    Ok(thumbnail_path)
}
