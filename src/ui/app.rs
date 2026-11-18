use dioxus::prelude::*;

use crate::{
    app_state::AppState,
    database::sqlite,
};

#[component]
pub fn App() -> Element {
    let state = use_context::<AppState>();

    let medias = use_resource(move || {
        let pool = state.pool.clone();

        async move {
            sqlite::get_all_media(&pool)
                .await
                .unwrap_or_default()
        }
    });

    rsx! {
        div {

            h1 {
                "Rust Media Library"
            }

            p {
                "Médias présents dans SQLite :"
            }

            if let Some(list) = &*medias.read_unchecked() {

                div {
                    style: "
                        display:flex;
                        flex-wrap:wrap;
                        gap:16px;
                        padding:16px;
                    ",

                    for media in list.iter() {

                        div {
                            key: "{media.path}",

                            style: "
                                width:220px;
                                border:1px solid #444;
                                border-radius:8px;
                                overflow:hidden;
                                background:#f5f5f5;
                            ",

                            if let Some(data_uri) = &media.thumbnail_data {

                                img {
                                    src: "{data_uri}",

                                    style: "
                                        width:100%;
                                        height:180px;
                                        object-fit:cover;
                                    ",
                                }

                            } else {

                                div {
                                    style: "
                                        height:180px;
                                        display:flex;
                                        align-items:center;
                                        justify-content:center;
                                        background:#ddd;
                                        font-size:48px;
                                    ",

                                    if media.media_type == "image" {
                                        "🖼️"
                                    } else {
                                        "🎬"
                                    }
                                }
                            }

                            div {
                                style: "padding:8px;",

                                p {
                                    style: "
                                        margin:0;
                                        font-size:12px;
                                        word-break:break-word;
                                    ",

                                    "{media.filename}"
                                }
                            }
                        }
                    }
                }

            } else {

                p {
                    "Chargement..."
                }
            }
        }
    }
}