mod app_state;
mod database;
mod models;
mod services;
mod ui;

use services::scanner::scan_folder;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let pool = rt
        .block_on(database::sqlite::init_database())
        .expect("Impossible d'initialiser SQLite");

    let medias = scan_folder("C:\\Users\\euchi\\Pictures");
    let app_state = app_state::AppState { pool: pool.clone() };
    println!("{} médias trouvés", medias.len());

    rt.block_on(async {
        for media in &medias {
            database::sqlite::insert_media(&pool, media).await.unwrap();

            if media.media_type == "image" {
                match services::thumbnails::generate_thumbnail(&media.path) {
                    Ok(thumbnail_path) => {
                        database::sqlite::update_thumbnail(&pool, &media.path, &thumbnail_path)
                            .await
                            .unwrap();

                        println!("Miniature créée : {}", media.filename);
                    }

                    Err(e) => {
                        eprintln!("Erreur miniature {} : {}", media.filename, e);
                    }
                }
            }
        }

        let count = database::sqlite::media_count(&pool).await.unwrap_or(0);

        println!("Total médias en base : {}", count);
    });

    dioxus::LaunchBuilder::desktop()
        .with_context(app_state)
        .launch(ui::app::App);
}
