use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FolderViewModel {

    pub path: String,

    pub enabled: bool,

    pub media_count: i64,
}
#[derive(Props, Clone, PartialEq)]
pub struct FoldersPanelProps {
    pub folders: Vec<FolderViewModel>,

    pub on_delete: Callback<String>,

    pub on_toggle: Callback<(String, bool)>,
}  

#[component]
pub fn FoldersPanel(
    props: FoldersPanelProps,
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
                "📂 Dossiers surveillés"
            }

            for folder in &props.folders {

                div {

                    style: "
                        display:flex;
                        justify-content:space-between;
                        align-items:center;
                        margin-bottom:4px;
                    ",

                    div {

                        style: "
                            display:flex;
                            align-items:center;
                            gap:8px;
                        ",

                        input {

                            r#type: "checkbox",

                            checked: folder.enabled,

                            onchange: {
                                let path = folder.path.clone();
                                let callback = props.on_toggle.clone();

                                move |evt: FormEvent| {

                                    callback.call((
                                        path.clone(),
                                        evt.checked(),
                                    ));
                                }
                            }
                        }

                        span {
                            "{folder.path}"
                        }

                        span {

                            style: "
                                color:#666;
                                font-size:12px;
                            ",

                            "({folder.media_count})"
                        }
                    }

                    button {

                        onclick: {
                            let path = folder.path.clone();
                            let callback =
                                props.on_delete.clone();

                            move |_| {

                                callback.call(
                                    path.clone()
                                );
                            }
                        },

                        "❌"
                    }
                }
            }
        }
    }
}