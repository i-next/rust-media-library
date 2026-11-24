mod app_state;
mod database;
mod models;
mod services;
mod ui;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let pool = rt
        .block_on(database::sqlite::init_database())
        .expect("Impossible d'initialiser SQLite");

    rt.block_on(async {
        let folders = database::sqlite::get_folders(&pool)
            .await
            .unwrap_or_default();
        println!("Folders au démarrage = {:?}", folders);
        if folders.is_empty() {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                database::sqlite::add_folder(&pool, &folder.to_string_lossy())
                    .await
                    .unwrap();

                println!("Dossier initial ajouté : {}", folder.display());
            }
        }
    });

    let app_state = app_state::AppState { pool: pool.clone() };

    rt.block_on(async {
        if let Err(e) =
            services::library_service::rescan_all_folders(&pool, |_current, _total| {}).await
        {
            eprintln!("Erreur scan initial : {}", e);
        }

        let count = database::sqlite::media_count(&pool).await.unwrap_or(0);

        println!("Total médias en base : {}", count);
    });
    rt.block_on(async {
        database::sqlite::debug_sqlite_version(&pool).await.unwrap();
        database::sqlite::debug_tables(&pool).await.unwrap();
        database::sqlite::debug_media_search_count(&pool)
            .await
            .unwrap();
        database::sqlite::debug_fts_search(&pool).await.unwrap();
        database::sqlite::debug_media_search_schema(&pool)
            .await
            .unwrap();
        database::sqlite::debug_filename_search_count(&pool)
            .await
            .unwrap();
        database::sqlite::debug_tables(&pool)
            .await
            .unwrap();
    });
    services::storage_service
        ::ensure_storage_layout()
        .expect(
            "Impossible d'initialiser le stockage"
    );

    dioxus::LaunchBuilder::desktop()
        .with_context(app_state)
        .launch(ui::app::App);
}
