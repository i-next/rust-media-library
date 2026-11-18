use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            h1 { "Rust Media Library" }

            input {
                r#type: "text",
                placeholder: "Recherche..."
            }

            div {
                "Aucun média"
            }
        }
    }
}