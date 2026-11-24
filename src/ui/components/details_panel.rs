use dioxus::prelude::*;

use crate::models::media::MediaFile;

#[derive(Props, Clone, PartialEq)]
pub struct DetailsPanelProps {

    pub media: Option<MediaFile>,

   /*  pub tags: Vec<String>,

    pub ai_description: Option<String>, */
}

#[component]
pub fn DetailsPanel(
    props: DetailsPanelProps,
) -> Element {

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

            if let Some(media) =
                &props.media
            {

                h3 {

                    if media.favorite {
                        "⭐ "
                    }

                    "{media.filename}"
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
            }
            else {

                p {
                    "Sélectionne une miniature."
                }
            }
        }
    }
}
