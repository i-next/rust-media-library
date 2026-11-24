use anyhow::Result;
use sqlx::SqlitePool;

use crate::{
    database::sqlite,
    services::{scanner::scan_folder, thumbnails},
};

// #[derive(Clone)]
pub struct ScanProgress {
    pub current: usize,
    pub total: usize,
}

pub async fn rescan_library(
    pool: &SqlitePool,
    folder: &str,
    mut on_progress: impl FnMut(usize, usize),
) -> Result<()> {
    println!("Début du scan : {}", folder);

    let medias = scan_folder(folder);

    println!("{} médias trouvés", medias.len());

    let total = medias.len();

    for (index, media) in medias.iter().enumerate() {
        on_progress(index + 1, total);
        tokio::task::yield_now().await;
        sqlite::insert_media(pool, media).await?;

        if media.media_type == "image" {
            match thumbnails::generate_thumbnail(&media.path) {
                Ok(thumbnail_path) => {
                    sqlite::update_thumbnail(pool, &media.path, &thumbnail_path).await?;
                }

                Err(e) => {
                    eprintln!("Erreur miniature {} : {}", media.filename, e);
                }
            }
        }
        if media.media_type == "video" {
            println!("AVANT THUMB");

            match crate::services::video_thumbnails::generate_video_thumbnail(&media.path) {
                Ok(thumbnail_path) => {
                    sqlite::update_thumbnail(pool, &media.path, &thumbnail_path).await?;
                }

                Err(e) => {
                    eprintln!("Erreur miniature vidéo {} : {}", media.filename, e);
                }
            }
        }
    }
    let deleted = sqlite::delete_missing_media(pool).await?;

    println!("{} fichiers supprimés de l'index", deleted);
    println!("Scan terminé");

    Ok(())
}
pub async fn rescan_all_folders(
    pool: &SqlitePool,
    mut on_progress: impl FnMut(usize, usize),
) -> Result<()> {
    let folders = sqlite::get_folders(pool).await?;

    for folder in folders {
        if folder.enabled {
            rescan_library(pool, &folder.path, &mut on_progress).await?;
        }
    }

    Ok(())
}
