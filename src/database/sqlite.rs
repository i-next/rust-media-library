use crate::models::media::MediaFile;
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};
use std::fs;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_database() -> Result<SqlitePool> {
    let current = std::env::current_dir()?;

    fs::create_dir_all("storage")?;

    let db_file = current.join("storage").join("library.db");

    if !db_file.exists() {
        std::fs::File::create(&db_file)?;
    }

    let database_url = format!("sqlite://{}", db_file.to_string_lossy());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    match MIGRATOR.run(&pool).await {
        Ok(_) => println!("Toutes les migrations appliquées"),
        Err(e) => {
            eprintln!("Erreur migration : {e:?}");
            return Err(e.into());
        }
    }

    println!("Base SQLite initialisée");

    Ok(pool)
}
pub async fn insert_media(pool: &SqlitePool, media: &MediaFile) -> Result<()> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO media (
            path,
            filename,
            extension,
            media_type,
            file_size,
            width,
            height,
            duration
        )
        VALUES (?, ?, ?, ?, ?, ?, ?,?)
        "#,
    )
    .bind(&media.path)
    .bind(&media.filename)
    .bind(&media.extension)
    .bind(&media.media_type)
    .bind(media.file_size.map(|v| v as i64))
    .bind(media.width.map(|v| v as i64))
    .bind(media.height.map(|v| v as i64))
    .bind(media.duration)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn media_count(pool: &SqlitePool) -> Result<i64> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM media")
        .fetch_one(pool)
        .await?;

    Ok(count)
}
pub async fn get_all_media(pool: &SqlitePool) -> Result<Vec<MediaFile>> {
    let rows = sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            String,
            Option<String>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<f64>,
        ),
    >(
        r#"
        SELECT
            path,
            filename,
            extension,
            media_type,
            thumbnail_path,
            file_size,
            width,
            height,
            duration
        FROM media
        ORDER BY filename
        "#,
    )
    .fetch_all(pool)
    .await?;

    let current_dir = std::env::current_dir()?;

    Ok(rows
        .into_iter()
        .map(|r| {
            let thumbnail_path =
                r.4.map(|thumb| current_dir.join(thumb).to_string_lossy().to_string());

            let thumbnail_data = thumbnail_path.as_ref().and_then(|path| {
                fs::read(path).ok().map(|bytes| {
                    format!(
                        "data:image/jpeg;base64,{}",
                        general_purpose::STANDARD.encode(bytes)
                    )
                })
            });

            MediaFile {
                path: r.0,
                filename: r.1,
                extension: r.2,
                media_type: r.3,

                thumbnail_path,

                thumbnail_data,

                file_size: r.5.map(|v| v as u64),

                width: r.6.map(|v| v as u32),

                height: r.7.map(|v| v as u32),

                duration: r.8,
            }
        })
        .collect())
}

pub async fn update_thumbnail(
    pool: &SqlitePool,
    media_path: &str,
    thumbnail_path: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE media
        SET thumbnail_path = ?
        WHERE path = ?
        "#,
    )
    .bind(thumbnail_path)
    .bind(media_path)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn delete_missing_media(
    pool: &SqlitePool,
) -> anyhow::Result<u64> {

    let rows = sqlx::query_scalar::<_, String>(
        "SELECT path FROM media"
    )
    .fetch_all(pool)
    .await?;

    let mut deleted = 0;

    for path in rows {

        if !std::path::Path::new(&path).exists() {

            let result = sqlx::query(
                "DELETE FROM media WHERE path = ?"
            )
            .bind(&path)
            .execute(pool)
            .await?;

            deleted += result.rows_affected();
        }
    }

    Ok(deleted)
}