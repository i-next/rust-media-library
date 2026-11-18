mod database;
mod ui;

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            database::sqlite::init_database()
                .await
                .expect("Impossible d'initialiser SQLite");
        });

    dioxus::launch(ui::app::App);
}