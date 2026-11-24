use dioxus::prelude::*;

use crate::models::media::MediaFile;
use crate::ui::components::media_card::MediaCard;

#[derive(Props, Clone, PartialEq)]
pub struct MediaGridProps {

    pub medias: Vec<MediaFile>,

    pub tagged_paths: Vec<String>,

    pub search_text: String,

    pub selected_tag: Option<String>,

    pub favorites_only: bool,

    pub on_select: Callback<MediaFile>,
}
fn media_matches_filters(
    media: &MediaFile,
    search_text: &str,
    tagged_paths: &[String],
    selected_tag: &Option<String>,
    favorites_only: bool,
) -> bool {

    let search =
        search_text.to_lowercase();

    let matches_name =

        search.is_empty()

        ||

        media.filename
            .to_lowercase()
            .contains(&search)

        ||

        media.path
            .to_lowercase()
            .contains(&search)

        ||

        media.extension
            .to_lowercase()
            .contains(&search);

    let matches_tag =
        tagged_paths.contains(&media.path);

    let matches_search =
        matches_name || matches_tag;

    let matches_favorite =
        !favorites_only
        || media.favorite;

    let matches_selected_tag =

        selected_tag.is_none()

        ||

        tagged_paths.contains(
            &media.path
        );

    matches_search
        &&
    matches_favorite
        &&
    matches_selected_tag
}
#[component]
pub fn MediaGrid(
    props: MediaGridProps,
) -> Element {

    rsx! {

        div {

            style: "
                display:flex;
                flex-wrap:wrap;
                gap:16px;
            ",

            for media in props.medias
                .iter()
                .filter(|media| {

                    media_matches_filters(
                        media,

                        &props.search_text,

                        &props.tagged_paths,

                        &props.selected_tag,

                        props.favorites_only,
                    )
                })
            {

                MediaCard {

                    media:
                        media.clone(),

                    on_select:
                        props.on_select.clone(),
                }
            }
        }
    }
}