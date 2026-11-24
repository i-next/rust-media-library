use dioxus::prelude::*;
use crate::models::media::MediaFile;

#[derive(Props, Clone, PartialEq)]
pub struct MediaCardProps {

    pub media: MediaFile,

    pub on_select: Callback<MediaFile>,
}

#[component]
pub fn MediaCard(
    props: MediaCardProps,
) -> Element {

    let media =
        props.media.clone();

    rsx! {

        div {

            key: "{media.path}",

            style: "
                position:relative;
                width:220px;
                border:1px solid #444;
                border-radius:8px;
                overflow:hidden;
                cursor:pointer;
                background:#f5f5f5;
            ",

            onclick: {

                let callback =
                    props.on_select.clone();

                let media =
                    media.clone();

                move |_| {

                    callback.call(
                        media.clone()
                    );
                }
            },

            ondoubleclick: {

                let path =
                    media.path.clone();

                move |_| {

                    let _ =
                        open::that(
                            path.clone()
                        );
                }
            },

            if media.favorite {

                div {

                    style: "
                        position:absolute;
                        top:8px;
                        right:8px;
                        z-index:10;
                        font-size:24px;
                        text-shadow:0 0 3px black;
                    ",

                    "⭐"
                }
            }

            if let Some(data_uri) =
                &media.thumbnail_data
            {

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

                    if media.media_type
                        == "image"
                    {
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