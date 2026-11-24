use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TagViewModel {

    pub name: String,

    pub count: i64,
}

#[derive(Props, Clone, PartialEq)]
pub struct TagsPanelProps {

    pub tags: Vec<TagViewModel>,

    pub selected_tag: Option<String>,

    pub on_select: Callback<String>,

    pub on_clear: Callback<()>,
}

#[component]
pub fn TagsPanel(
    props: TagsPanelProps,
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
                "🏷️ Tags"
            }

            for tag in &props.tags {

                div {

                    style: {

                        let selected =
                            props.selected_tag
                                .as_ref()
                                .map(|t| t == &tag.name)
                                .unwrap_or(false);

                        if selected {

                            "
                            cursor:pointer;
                            padding:4px;
                            background:#3b82f6;
                            color:white;
                            border-radius:4px;
                            "
                        } else {

                            "
                            cursor:pointer;
                            padding:4px;
                            "
                        }
                    },

                    onclick: {

                        let callback =
                            props.on_select.clone();

                        let tag_name =
                            tag.name.clone();

                        move |_| {

                            callback.call(
                                tag_name.clone()
                            );
                        }
                    },

                    "{tag.name} ({tag.count})"
                }
            }

            button {

                onclick: move |_| {

                    props.on_clear.call(());
                },

                "Effacer filtre"
            }
        }
    }
}