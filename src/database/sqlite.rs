use crate::models::folder::WatchedFolder;
use crate::models::media::MediaFile;
use crate::models::tag::Tag;
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};
use std::fs;
use crate::models::tag::TagCount;
use crate::models::library_stats::LibraryStats;
use crate::services::ai_service::is_allowed_tag;
use crate::services::ai_service::normalize_tag;
use std::collections::HashSet;
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
            duration,
            favorite
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
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
    .bind(media.favorite)
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO media_filename_search (
            path,
            filename,
            extension,
            media_type
        )
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&media.path)
    .bind(&media.filename)
    .bind(&media.extension)
    .bind(&media.media_type)
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO media_filename_search (
            path,
            filename,
            extension,
            media_type
        )
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&media.path)
    .bind(&media.filename)
    .bind(&media.extension)
    .bind(&media.media_type)
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
            i64,
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
            duration,
            favorite
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
                favorite: false,
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
pub async fn delete_missing_media(pool: &SqlitePool) -> anyhow::Result<u64> {
    let rows = sqlx::query_scalar::<_, String>("SELECT path FROM media")
        .fetch_all(pool)
        .await?;

    let mut deleted = 0;

    for path in rows {
        if !std::path::Path::new(&path).exists() {
            sqlx::query("DELETE FROM media_filename_search WHERE path = ?")
                .bind(&path)
                .execute(pool)
                .await?;
            let result = sqlx::query("DELETE FROM media WHERE path = ?")
                .bind(&path)
                .execute(pool)
                .await?;

            deleted += result.rows_affected();
        }
    }

    Ok(deleted)
}
pub async fn add_folder(pool: &SqlitePool, path: &str) -> Result<()> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO folders(path)
        VALUES(?)
        "#,
    )
    .bind(path)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_folders(pool: &SqlitePool) -> Result<Vec<WatchedFolder>> {
    let rows = sqlx::query_as::<_, (String, bool, i64)>(
        r#"
        SELECT
            f.path,
            f.enabled,

            (
                SELECT COUNT(*)
                FROM media m
                WHERE m.path LIKE f.path || '%'
            ) AS media_count

        FROM folders f
        ORDER BY f.path
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| WatchedFolder {
            path: r.0,
            enabled: r.1,
            media_count: r.2,
        })
        .collect())
}
pub async fn remove_folder(pool: &SqlitePool, path: &str) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM folders
        WHERE path = ?
        "#,
    )
    .bind(path)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_visible_media(pool: &SqlitePool) -> Result<Vec<MediaFile>> {
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
            i64,
        ),
    >(
        r#"
        SELECT
            m.path,
            m.filename,
            m.extension,
            m.media_type,
            m.thumbnail_path,
            m.file_size,
            m.width,
            m.height,
            m.duration,
            m.favorite
        FROM media m
        WHERE EXISTS (
            SELECT 1
            FROM folders f
            WHERE
                f.enabled = 1
                AND m.path LIKE f.path || '%'
        )
        ORDER BY m.filename
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
                favorite: r.9 != 0,
            }
        })
        .collect())
}
pub async fn set_folder_enabled(pool: &SqlitePool, path: &str, enabled: bool) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE folders
        SET enabled = ?
        WHERE path = ?
        "#,
    )
    .bind(enabled)
    .bind(path)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn search_media(pool: &SqlitePool, query: &str) -> Result<Vec<MediaFile>> {
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
            i64,
        ),
    >(
        r#"
        SELECT
            m.path,
            m.filename,
            m.extension,
            m.media_type,
            m.thumbnail_path,
            m.file_size,
            m.width,
            m.height,
            m.duration,
            m.favorite,
        FROM media m
        JOIN media_filename_search s
            ON s.path = m.path
        WHERE media_filename_search MATCH ?
        ORDER BY m.filename
        "#,
    )
    .bind(query)
    .fetch_all(pool)
    .await?;

    let current_dir = std::env::current_dir()?;

    Ok(rows
        .into_iter()
        .map(|r| {
            let thumbnail_path =
                r.4.map(|thumb| current_dir.join(thumb).to_string_lossy().to_string());

            let thumbnail_data = thumbnail_path.as_ref().and_then(|path| {
                std::fs::read(path).ok().map(|bytes| {
                    format!(
                        "data:image/jpeg;base64,{}",
                        base64::engine::general_purpose::STANDARD.encode(bytes)
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
                favorite: r.9 != 0,
            }
        })
        .collect())
}
pub async fn debug_sqlite_version(pool: &SqlitePool) -> Result<()> {
    let version: String = sqlx::query_scalar("SELECT sqlite_version();")
        .fetch_one(pool)
        .await?;

    println!("SQLite version = {}", version);

    Ok(())
}
/* pub async fn debug_tables(pool: &SqlitePool) -> Result<()> {
    let tables: Vec<String> = sqlx::query_scalar(
        r#"
            SELECT name
            FROM sqlite_master
            WHERE type='table'
            ORDER BY name
            "#,
    )
    .fetch_all(pool)
    .await?;

    println!("Tables = {:?}", tables);

    Ok(())
} */
pub async fn debug_media_search_count(pool: &SqlitePool) -> Result<()> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM media_search")
        .fetch_one(pool)
        .await?;

    println!("FTS5 count = {}", count);

    Ok(())
}
pub async fn debug_fts_search(pool: &SqlitePool) -> Result<()> {
    let rows: Vec<String> = sqlx::query_scalar(
        r#"
            SELECT filename
            FROM media_search
            WHERE media_search MATCH 'jpg'
            LIMIT 10
            "#,
    )
    .fetch_all(pool)
    .await?;

    println!("FTS result = {:?}", rows);

    Ok(())
}
pub async fn debug_media_search_schema(pool: &SqlitePool) -> Result<()> {
    let rows: Vec<String> = sqlx::query_scalar(
        r#"
            SELECT sql
            FROM sqlite_master
            WHERE name = 'media_search'
            "#,
    )
    .fetch_all(pool)
    .await?;

    println!("Schema media_search = {:?}", rows);

    Ok(())
}
pub async fn debug_filename_search_count(pool: &SqlitePool) -> Result<()> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM media_filename_search")
        .fetch_one(pool)
        .await?;

    println!("Filename FTS count = {}", count);

    Ok(())
}
pub async fn set_favorite(
    pool: &SqlitePool,
    path: &str,
    favorite: bool,
) -> Result<()> {

    sqlx::query(
        r#"
        UPDATE media
        SET favorite = ?
        WHERE path = ?
        "#
    )
    .bind(favorite)
    .bind(path)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn debug_tables(
    pool: &SqlitePool,
) -> Result<()> {

    let tables: Vec<String> =
        sqlx::query_scalar(
            r#"
            SELECT name
            FROM sqlite_master
            WHERE type='table'
            ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await?;

    println!("Tables = {:?}", tables);

    Ok(())
}
pub async fn add_tag(
    pool: &SqlitePool,
    name: &str,
) -> Result<()> {

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO tags(name)
        VALUES(?)
        "#
    )
    .bind(name.trim())
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_tags(
    pool: &SqlitePool,
) -> Result<Vec<Tag>> {

    let rows = sqlx::query_as::<_, (i64, String)>(
        r#"
        SELECT id, name
        FROM tags
        ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(
        rows.into_iter()
            .map(|r| Tag {
                id: r.0,
                name: r.1,
            })
            .collect()
    )
}
pub async fn assign_tag(
    pool: &SqlitePool,
    media_path: &str,
    tag_name: &str,
) -> Result<()> {

    add_tag(pool, tag_name).await?;

    let media_id: i64 =
        sqlx::query_scalar(
            "SELECT id FROM media WHERE path = ?"
        )
        .bind(media_path)
        .fetch_one(pool)
        .await?;

    let tag_id: i64 =
        sqlx::query_scalar(
            "SELECT id FROM tags WHERE name = ?"
        )
        .bind(tag_name)
        .fetch_one(pool)
        .await?;

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO media_tags(
            media_id,
            tag_id
        )
        VALUES(?, ?)
        "#
    )
    .bind(media_id)
    .bind(tag_id)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn unassign_tag(
    pool: &SqlitePool,
    media_path: &str,
    tag_name: &str,
) -> Result<()> {

    let media_id: i64 =
        sqlx::query_scalar(
            "SELECT id FROM media WHERE path = ?"
        )
        .bind(media_path)
        .fetch_one(pool)
        .await?;

    let tag_id: i64 =
        sqlx::query_scalar(
            "SELECT id FROM tags WHERE name = ?"
        )
        .bind(tag_name)
        .fetch_one(pool)
        .await?;

    sqlx::query(
        r#"
        DELETE FROM media_tags
        WHERE media_id = ?
          AND tag_id = ?
        "#
    )
    .bind(media_id)
    .bind(tag_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_media_tags(
    pool: &SqlitePool,
    media_path: &str,
) -> Result<Vec<Tag>> {

    let rows =
        sqlx::query_as::<_, (i64, String)>(
            r#"
            SELECT
                t.id,
                t.name
            FROM tags t
            JOIN media_tags mt
                ON mt.tag_id = t.id
            JOIN media m
                ON m.id = mt.media_id
            WHERE m.path = ?
            ORDER BY t.name
            "#
        )
        .bind(media_path)
        .fetch_all(pool)
        .await?;

    Ok(
        rows.into_iter()
            .map(|r| Tag {
                id: r.0,
                name: r.1,
            })
            .collect()
    )
}
pub async fn delete_tag(
    pool: &SqlitePool,
    tag_id: i64,
) -> Result<()> {

    sqlx::query(
        "DELETE FROM tags WHERE id = ?"
    )
    .bind(tag_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn media_has_tag(
    pool: &SqlitePool,
    media_path: &str,
    search: &str,
) -> Result<bool> {

    let count: i64 =
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM tags t
            JOIN media_tags mt
                ON mt.tag_id = t.id
            JOIN media m
                ON m.id = mt.media_id
            WHERE
                m.path = ?
                AND LOWER(t.name)
                    LIKE LOWER(?)
            "#
        )
        .bind(media_path)
        .bind(format!("%{}%", search))
        .fetch_one(pool)
        .await?;

    Ok(count > 0)
}
pub async fn search_media_paths_by_tag(
    pool: &SqlitePool,
    search: &str,
) -> Result<Vec<String>> {

    let rows =
        sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT m.path
            FROM media m
            JOIN media_tags mt
                ON mt.media_id = m.id
            JOIN tags t
                ON t.id = mt.tag_id
            WHERE LOWER(t.name)
                LIKE LOWER(?)
            "#
        )
        .bind(format!("%{}%", search))
        .fetch_all(pool)
        .await?;

    Ok(rows)
}
pub async fn get_tag_counts(
    pool: &SqlitePool,
) -> Result<Vec<TagCount>> {

    let rows = sqlx::query_as::<_, (String, i64)>(
        r#"
        SELECT
            t.name,
            COUNT(mt.media_id)
        FROM tags t
        LEFT JOIN media_tags mt
            ON mt.tag_id = t.id
        GROUP BY t.id, t.name
        ORDER BY COUNT(mt.media_id) DESC,
                 t.name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(
        rows.into_iter()
            .map(|r| TagCount {
                name: r.0,
                count: r.1,
            })
            .collect()
    )
}
pub async fn get_library_stats(
    pool: &SqlitePool,
) -> Result<LibraryStats> {

    let total_media: i64 =
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM media"
        )
        .fetch_one(pool)
        .await?;

    let total_images: i64 =
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM media WHERE media_type='image'"
        )
        .fetch_one(pool)
        .await?;

    let total_videos: i64 =
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM media WHERE media_type='video'"
        )
        .fetch_one(pool)
        .await?;

    let total_favorites: i64 =
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM media WHERE favorite=1"
        )
        .fetch_one(pool)
        .await?;

    let total_tags: i64 =
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM tags"
        )
        .fetch_one(pool)
        .await?;

    let total_size: i64 =
        sqlx::query_scalar(
            "SELECT COALESCE(SUM(file_size),0) FROM media"
        )
        .fetch_one(pool)
        .await?;

    Ok(LibraryStats {
        total_media,
        total_images,
        total_videos,
        total_favorites,
        total_tags,
        total_size,
    })
}

pub async fn save_ai_analysis(
    pool: &SqlitePool,

    media_id: i64,

    model_name: &str,

    analysis_type: &str,

    result_text: &str,

    raw_json: Option<&str>,
) -> Result<()> {

    sqlx::query(
        r#"
        INSERT INTO ai_analysis (
            media_id,
            model_name,
            analysis_type,
            result_text,
            raw_json
        )
        VALUES (?, ?, ?, ?, ?)
        "#
    )
    .bind(media_id)
    .bind(model_name)
    .bind(analysis_type)
    .bind(result_text)
    .bind(raw_json)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_latest_analysis(
    pool: &SqlitePool,

    media_id: i64,

    analysis_type: &str,
) -> Result<Option<String>> {

    let result =
        sqlx::query_scalar::<_, String>(
            r#"
            SELECT result_text
            FROM ai_analysis
            WHERE media_id = ?
              AND analysis_type = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(media_id)
        .bind(analysis_type)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}
pub async fn get_media_id(
    pool: &SqlitePool,
    path: &str,
) -> Result<Option<i64>> {

    let id =
        sqlx::query_scalar::<_, i64>(
            r#"
            SELECT id
            FROM media
            WHERE path = ?
            "#
        )
        .bind(path)
        .fetch_optional(pool)
        .await?;

    Ok(id)
}
pub async fn apply_ai_tags(
    pool: &SqlitePool,
    media_path: &str,
    tags: &[String],
) -> Result<()> {

    let mut unique =
    HashSet::new();

for tag in tags {

    let clean_tag =
        normalize_tag(tag);

    if clean_tag.is_empty() {
        continue;
    }

    if !is_allowed_tag(
        &clean_tag
    ) {
        continue;
    }

    if !unique.insert(
        clean_tag.clone()
    ) {
        continue;
    }

    assign_tag(
        pool,
        media_path,
        &clean_tag,
    )
    .await?;
}

    Ok(())
}
