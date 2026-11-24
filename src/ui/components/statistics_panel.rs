use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatisticsPanelProps {

    pub total_media: i64,

    pub total_images: i64,

    pub total_videos: i64,

    pub total_favorites: i64,

    pub total_tags: i64,

    pub total_size: i64,
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
pub fn StatisticsPanel(
    props: StatisticsPanelProps,
) -> Element {

    rsx! {

        div {

            style: "
                margin-bottom:20px;
                padding:10px;
                border:1px solid #ccc;
                border-radius:8px;
            ",

            h3 {
                "📊 Bibliothèque"
            }

            p {
                "Médias : {props.total_media}"
            }

            p {
                "Images : {props.total_images}"
            }

            p {
                "Vidéos : {props.total_videos}"
            }

            p {
                "Favoris : {props.total_favorites}"
            }

            p {
                "Tags : {props.total_tags}"
            }

            p {
                "Taille totale : {format_file_size(props.total_size as u64)}"
            }
        }
    }
}