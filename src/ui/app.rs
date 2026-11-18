use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    use_effect(move || {
        spawn(async move {
            if let Err(e) = crate::database::sqlite::init_database().await {
                eprintln!("Erreur SQLite : {e}");
            }
        });
    });

    rsx! {
        div {
            h1 { "Rust Media Library" }
        }
    }
}