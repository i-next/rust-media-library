use std::fs;
use std::path::Path;

use image::image_dimensions;
use walkdir::WalkDir;

use crate::models::media::MediaFile;

use std::process::Command;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "bmp"];

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "wmv", "webm"];

pub fn scan_folder(folder: &str) -> Vec<MediaFile> {
    println!("Scan du dossier : {}", folder);

    let mut medias = Vec::new();

    for entry in WalkDir::new(folder)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        println!("{}", entry.path().display());

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(media) = create_media(path) {
            medias.push(media);
        }
    }

    println!("{} médias trouvés", medias.len());

    medias
}

fn create_media(path: &Path) -> Option<MediaFile> {
    let extension = path.extension()?.to_string_lossy().to_lowercase();

    let media_type = if IMAGE_EXTENSIONS.contains(&extension.as_str()) {
        "image"
    } else if VIDEO_EXTENSIONS.contains(&extension.as_str()) {
        "video"
    } else {
        return None;
    };

    let filename = path.file_name()?.to_string_lossy().to_string();

    if filename.trim().is_empty() {
        return None;
    }

    let metadata = fs::metadata(path).ok();

    let file_size = metadata.map(|m| m.len());

    let (width, height) = if media_type == "image" {
        match image_dimensions(path) {
            Ok((w, h)) => (Some(w), Some(h)),
            Err(_) => (None, None),
        }
    } else {
        (None, None)
    };

    let duration = if media_type == "video" {
        get_video_duration(path)
    } else {
        None
    };

    Some(MediaFile {
        path: path.to_string_lossy().to_string(),
        filename,
        extension,
        media_type: media_type.to_string(),

        thumbnail_path: None,
        thumbnail_data: None,

        file_size,
        width,
        height,

        duration,
        favorite: false,
    })
}
fn get_video_duration(path: &Path) -> Option<f64> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(path)
        .output()
        .ok()?;

    let duration = String::from_utf8_lossy(&output.stdout);

    duration.trim().parse::<f64>().ok()
}
