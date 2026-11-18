use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;

pub async fn init_database() -> Result<SqlitePool> {
    let current = std::env::current_dir()?;

    println!("Répertoire courant : {:?}", current);

    fs::create_dir_all("storage")?;

    let db_file = current.join("storage").join("library.db");

    println!("Fichier DB : {:?}", db_file);

    // Création manuelle du fichier pour vérifier les droits
    if !db_file.exists() {
        std::fs::File::create(&db_file)?;
        println!("Fichier créé");
    }

    let database_url = format!("sqlite://{}", db_file.to_string_lossy());

    println!("URL SQLx : {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connexion OK");

    Ok(pool)
}
