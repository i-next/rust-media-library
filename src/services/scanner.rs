use std::path::Path;
use walkdir::WalkDir;

use crate::models::media::MediaFile;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "bmp"];

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "wmv", "webm"];

pub fn scan_folder(folder: &str) -> Vec<MediaFile> {
    let mut medias = Vec::new();

    for entry in WalkDir::new(folder)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(media) = create_media(path) {
            medias.push(media);
        }
    }

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

    Some(MediaFile {
    path: path.to_string_lossy().to_string(),
    filename,
    extension,
    media_type: media_type.to_string(),
    thumbnail_path: None,
    thumbnail_data: None,
})
}
