use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub fn media_hash(
    media_path: &str,
) -> String {

    let mut hasher =
        Sha256::new();

    hasher.update(
        media_path.as_bytes()
    );

    format!(
        "{:x}",
        hasher.finalize()
    )
}

pub fn ensure_storage_layout() -> std::io::Result<()> {

    fs::create_dir_all("storage/thumbnails")?;
    fs::create_dir_all("storage/previews")?;
    fs::create_dir_all("storage/ai")?;
    fs::create_dir_all("storage/embeddings")?;

    Ok(())
}

pub fn thumbnail_path(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("thumbnails")
        .join(bucket)
        .join(format!("{hash}.webp"))
}

pub fn preview_path(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("previews")
        .join(bucket)
        .join(format!("{hash}.webp"))
}

pub fn ai_json_path(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("ai")
        .join(bucket)
        .join(format!("{hash}.json"))
}

pub fn embedding_path(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("embeddings")
        .join(bucket)
        .join(format!("{hash}.bin"))
}

pub fn ensure_parent(
    path: &Path,
) -> std::io::Result<()> {

    if let Some(parent) =
        path.parent()
    {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}
pub fn image_thumbnail_path(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("thumbnails")
        .join(bucket)
        .join(format!("{hash}.webp"))
}
pub fn video_thumbnail_path(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("thumbnails")
        .join(bucket)
        .join(format!("{hash}.jpg"))
}
pub fn video_frames_dir(
    media_path: &str,
) -> PathBuf {

    let hash =
        media_hash(media_path);

    let bucket =
        &hash[..2];

    Path::new("storage")
        .join("frames")
        .join(bucket)
        .join(hash)
}