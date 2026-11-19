use dioxus::prelude::*;

use crate::{app_state::AppState, database::sqlite};

fn format_file_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let size_f = size as f64;

    if size_f >= GB {
        format!("{:.2} Go", size_f / GB)
    } else if size_f >= MB {
        format!("{:.2} Mo", size_f / MB)
    } else if size_f >= KB {
        format!("{:.2} Ko", size_f / KB)
    } else {
        format!("{} octets", size)
    }
}

fn format_duration(seconds: f64) -> String {
    let total = seconds.round() as u64;

    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;

    if h > 0 {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        format!("{m:02}:{s:02}")
    }
}

#[component]
pub fn App() -> Element {
    let state = use_context::<AppState>();

    use crate::models::media::MediaFile;

    let mut selected_media = use_signal(|| None::<MediaFile>);

    let medias = use_resource(move || {
        let pool = state.pool.clone();

        async move { sqlite::get_all_media(&pool).await.unwrap_or_default() }
    });

    let selected = selected_media();

    rsx! {
                div {
                    style: "display:flex; height:100vh;",

                    // Galerie
                    div {
                        style: "
                    flex:1;
                    overflow:auto;
                    padding:16px;
                ",

                        h1 { "Rust Media Library" }

                        p { "Médias présents dans SQLite :" }

                        if let Some(list) = &*medias.read_unchecked() {

                            div {
                                style: "
                            display:flex;
                            flex-wrap:wrap;
                            gap:16px;
                        ",

                                for media in list.iter() {

                                    div {
                                        key: "{media.path}",

                                        style: "
                                    width:220px;
                                    border:1px solid #444;
                                    border-radius:8px;
                                    overflow:hidden;
                                    cursor:pointer;
                                    background:#f5f5f5;
                                ",

                                        onclick: {
                                            let media_clone = media.clone();

                                            move |_| {
                                                selected_media.set(Some(media_clone.clone()));
                                            }
                                        },

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

                    // Panneau de détails
                    div {
                        style: "
                    width:350px;
                    padding:16px;
                    border-left:1px solid #444;
                    overflow:auto;
                ",

                        h2 {
                            "Détails"
                        }

                        if let Some(media) = selected {

            h3 {
                "{media.filename}"
            }

            hr {}

            p {
                strong { "Type :" }
                " "
                "{media.media_type}"
            }

            p {
                strong { "Extension :" }
                " "
                "{media.extension}"
            }

            p {
                strong { "Taille :" }
                " "

                if let Some(size) = media.file_size {
                     "{format_file_size(size)}"
                } else {
                    "Inconnue"
                }
            }

            p {
                strong { "Dimensions :" }
                " "

                if let (Some(w), Some(h)) =
                    (media.width, media.height)
                {
                    "{w} × {h}"
                } else {
                    "-"
                }
            }

            p {
                strong { "Chemin :" }
            }

            p {
                style: "
            word-break:break-all;
            font-size:12px;
        ",

                "{media.path}"
            }

            p {
                strong { "Miniature générée :" }
                " "
                if media.thumbnail_data.is_some() {
                    "Oui"
                } else {
                    "Non"
                }
            }

            if let Some(data_uri) = &media.thumbnail_data {

                img {
                    src: "{data_uri}",

                    style: "
                width:100%;
                border:1px solid #ccc;
                border-radius:8px;
                margin-top:16px;
            ",
                }
            }

            if media.media_type == "video" {

        p {
            strong { "Durée :" }
            " "

            if let Some(duration) = media.duration {
                "{format_duration(duration)}"
            } else {
                "Inconnue"
            }
        }
    }

        } else {

            p {
                "Sélectionne une miniature."
            }
        }
                    }
                }
            }
}
