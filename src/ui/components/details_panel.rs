use dioxus::prelude::*;

use crate::models::media::MediaFile;

#[derive(Props, Clone, PartialEq)]
pub struct DetailsPanelProps {

    pub media: Option<MediaFile>,

    pub media_tags: Vec<String>,

    pub all_tags: Vec<String>,

    pub on_toggle_favorite: Callback<String>,

    pub on_add_tag: Callback<String>,

    pub on_assign_tag: Callback<String>,

    pub on_remove_tag: Callback<String>,

    pub ai_description: Option<String>,

    /* pub on_analyze: Callback<()>, */

   /*  pub tags: Vec<String>,

    pub ai_description: Option<String>, */
}

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

#[component]
pub fn DetailsPanel(
    props: DetailsPanelProps,
) -> Element {
    let mut new_tag =
        use_signal(String::new);

    let mut selected_tag =
        use_signal(String::new);
    rsx! {
        
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
            /* button {

                onclick: move |_| {

                    props.on_analyze.call(());
                },

                "🤖 Analyser"
            } */

            h3 {
                "🤖 Analyse IA"
            }

            if let Some(description) =
                &props.ai_description
            {
                p {
                    "{description}"
                }
            }
            else
            {
                p {
                    "Aucune analyse IA disponible."
                }
            }

            if let Some(media) =
                &props.media
            {

                h3 {

                    if media.favorite {
                        "⭐ "
                    }

                    "{media.filename}"
                }
                button {

                    onclick: {

                        let path =
                            media.path.clone();

                        let callback =
                            props.on_toggle_favorite.clone();

                        move |_| {

                            callback.call(
                                path.clone()
                            );
                        }
                    },

                    style: "
                        width:100%;
                        margin-bottom:10px;
                        padding:8px;
                    ",

                    if media.favorite {

                        "⭐ Retirer des favoris"

                    } else {

                        "☆ Ajouter aux favoris"
                    }
                }

                p {

                    strong {
                        "Type :"
                    }

                    " "

                    "{media.media_type}"
                }

                p {

                    strong {
                        "Extension :"
                    }

                    " "

                    "{media.extension}"
                }

                p {

                    strong {
                        "Chemin :"
                    }
                }

                p {

                    style: "
                        word-break:break-all;
                        font-size:12px;
                    ",

                    "{media.path}"
                }

                p {
                    strong {
                        "Taille :"
                    }

                    " "

                    if let Some(size) =
                        media.file_size
                    {
                        "{format_file_size(size)}"
                    } else {
                        "Inconnue"
                    }
                }

                p {
                    strong {
                        "Dimensions :"
                    }

                    " "

                    if let (Some(w), Some(h)) =
                        (media.width, media.height)
                    {
                        "{w} × {h}"
                    } else {
                        "-"
                    }
                }
                hr {}

                h3 {
                    "🏷️ Tags"
                }
                input {

                    value: "{new_tag()}",

                    placeholder: "Nouveau tag",

                    oninput: move |evt| {
                        new_tag.set(
                            evt.value()
                        );
                    }
                }

                button {

                    onclick: {

                        let callback =
                            props.on_add_tag.clone();

                        move |_| {

                            let tag =
                                new_tag();

                            if !tag.trim().is_empty() {

                                callback.call(
                                    tag.clone()
                                );

                                new_tag.set(
                                    String::new()
                                );
                            }
                        }
                    },

                    "➕ Créer"
                }

                select {

                    onchange: move |evt| {

                        selected_tag.set(
                            evt.value()
                        );
                    },

                    option {
                        value: "",
                        "-- Tag existant --"
                    }

                    for tag in &props.all_tags {

                        option {

                            value: "{tag}",

                            "{tag}"
                        }
                    }
                }

                button {

                    onclick: {

                        let callback =
                            props.on_assign_tag.clone();

                        move |_| {

                            let tag =
                                selected_tag();

                            if !tag.is_empty() {

                                callback.call(
                                    tag.clone()
                                );
                            }
                        }
                    },

                    "🔗 Associer"
                }

                if props.media_tags.is_empty() {

                    p {
                        style: "color:#888;",
                        "Aucun tag"
                    }

                } else {

                    div {

                        style: "
                            display:flex;
                            flex-wrap:wrap;
                            gap:6px;
                            margin-bottom:12px;
                        ",

                        for tag in &props.media_tags {

                            div {

                                style: "
                                    display:flex;
                                    align-items:center;
                                    gap:4px;
                                    background:#e5e7eb;
                                    padding:4px 8px;
                                    border-radius:12px;
                                ",

                                span {
                                    "{tag}"
                                }

                                button {

                                    onclick: {
                                        let tag =
                                            tag.clone();

                                        let callback =
                                            props.on_remove_tag.clone();

                                        move |_| {

                                            callback.call(
                                                tag.clone()
                                            );
                                        }
                                    },

                                    "❌"
                                }
                            }
                        }
                    }
                }
                if let Some(data_uri) =
                    &media.thumbnail_data
                {
                    img {

                        src: "{data_uri}",

                        style: "
                            width:100%;
                            max-height:250px;
                            object-fit:contain;
                        ",
                    }
                }
            }
            else {

                p {
                    "Sélectionne une miniature."
                }
            }
        }
    }
}
