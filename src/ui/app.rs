use dioxus::prelude::*;

use crate::services::library_service;
use crate::{app_state::AppState, database::sqlite};

/* use crate::ui::components::statistics_panel::StatisticsPanel; */
use crate::ui::components::folders_panel::{
    FolderViewModel,
    FoldersPanel,
};

use crate::ui::components::tags_panel::{
    TagViewModel,
    TagsPanel,
};
use crate::ui::components::media_grid::MediaGrid;
use crate::ui::components::details_panel::DetailsPanel;
/* fn format_file_size(size: u64) -> String {
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
} */

/* fn format_duration(seconds: f64) -> String {
    let total = seconds.round() as u64;

    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;

    if h > 0 {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        format!("{m:02}:{s:02}")
    }
} */

#[component]
pub fn App() -> Element {
    let state = use_context::<AppState>();
    let mut search_text = use_signal(String::new);
    use crate::models::media::MediaFile;

    let selected_media = use_signal(|| None::<MediaFile>);
    let pool = state.pool.clone();


    let scan_progress = use_signal(|| (0usize, 0usize));
    let refresh_counter = use_signal(|| 0u64);

    let is_scanning = use_signal(|| false);
    let pool_for_medias = pool.clone();
    let pool_for_folders = pool.clone();
    let pool_for_rescan = pool.clone();
    let pool_for_tag_search = pool.clone();
    let medias = use_resource(move || {
        let pool = pool_for_medias.clone();

        let search = search_text();

        let _refresh = refresh_counter();

        async move {
            if search.trim().is_empty() {
                sqlite::get_visible_media(&pool).await.unwrap_or_default()
            } else {
                sqlite::search_media(&pool, &search)
                    .await
                    .unwrap_or_default()
            }
        }
    });

    let mut show_favorites =
    use_signal(|| false);
    let mut sort_mode =
        use_signal(|| "name".to_string());

    let selected_tag =
        use_signal(|| None::<String>);

    let folders = use_resource(move || {
        let pool = pool_for_folders.clone();

        let _refresh = refresh_counter();

        async move { sqlite::get_folders(&pool).await.unwrap_or_default() }
    });
    let tag_matches = use_resource(move || {

        let pool =
            pool_for_tag_search.clone();

        let selected =
            selected_tag();

        async move {

            match selected {

                Some(tag) => {

                    sqlite::search_media_paths_by_tag(
                        &pool,
                        &tag,
                    )
                    .await
                    .unwrap_or_default()
                }

                None => Vec::<String>::new(),
            }
        }
    });
    let tagged_paths =
        tag_matches
            .read_unchecked()
            .as_ref()
            .cloned()
            .unwrap_or_default();



    let medias_sorted = if let Some(list) = &*medias.read_unchecked() {
        
    let mut medias = list.clone();

    match sort_mode().as_str() {

        "favorite" => {
            medias.sort_by_key(|m| !m.favorite);
        }

        "size" => {
            medias.sort_by_key(
                |m| std::cmp::Reverse(
                    m.file_size.unwrap_or(0)
                )
            );
        }

        "duration" => {
            medias.sort_by(|a, b| {
                b.duration
                    .partial_cmp(&a.duration)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        "extension" => {
            medias.sort_by(|a, b| {
                a.extension.cmp(&b.extension)
            });
        }

        _ => {
            medias.sort_by(|a, b| {
                a.filename.cmp(&b.filename)
            });
        }
    }

    medias

    } else {
        Vec::new()
    };


    let pool_for_tag_panel =
        pool.clone();

    let tags_panel = use_resource(move || {

        let pool =
            pool_for_tag_panel.clone();

        let _refresh =
            refresh_counter();

        async move {

            sqlite::get_tag_counts(&pool)
                .await
                .unwrap_or_default()
        }
    });
    /* let pool_for_stats =
        pool.clone(); */

/*     let stats = use_resource(move || {

        let pool =
            pool_for_stats.clone();

        let _refresh =
            refresh_counter();

        async move {

            sqlite::get_library_stats(&pool)
                .await
                .ok()
        }
    }); */



    let toggle_folder = Callback::new({

        let pool =
            pool_for_rescan.clone();

        let mut refresh =
            refresh_counter;

        move |(path, checked): (String, bool)| {

            let pool =
                pool.clone();

            spawn(async move {

                let _ =
                    sqlite::set_folder_enabled(
                        &pool,
                        &path,
                        checked,
                    )
                    .await;

                let current =
                    refresh();

                refresh.set(
                    current + 1
                );
            });
        }
    });
    let delete_folder = Callback::new({

        let pool =
            pool_for_rescan.clone();

        let mut refresh =
            refresh_counter;

        move |path: String| {

            let pool =
                pool.clone();

            spawn(async move {

                let _ =
                    sqlite::remove_folder(
                        &pool,
                        &path,
                    )
                    .await;

                let current =
                    refresh();

                refresh.set(
                    current + 1
                );
            });
        }
    });

    let select_media =
        Callback::new({

            let mut selected_media =
                selected_media;

            move |media: MediaFile| {

                selected_media.set(
                    Some(media)
                );
            }
        });

    let select_tag =
        Callback::new({

            let mut selected_tag =
                selected_tag;

            move |tag_name: String| {

                selected_tag.set(
                    Some(tag_name)
                );
            }
        });

    let clear_tag_filter =
        Callback::new({

            let mut selected_tag =
                selected_tag;

            move |_: ()| {

                selected_tag.set(None);
            }
        });
        let pool_for_media_tags =
            pool.clone();

        let media_tags = use_resource(move || {

            let pool =
             pool_for_media_tags.clone();

            let selected = selected_media();

            let refresh = refresh_counter();

            async move {

                let _ = refresh;

                match selected {

                    Some(media) => {

                        sqlite::get_media_tags(
                            &pool,
                            &media.path,
                        )
                        .await
                        .unwrap_or_default()
                    }

                    None => vec![],
                }
            }
        });

    let toggle_favorite =
    Callback::new({

        let pool =
            state.pool.clone();

        let mut refresh =
            refresh_counter;

        let mut selected_media_signal =
            selected_media;

        move |path: String| {

            let pool =
                pool.clone();

            spawn(async move {

                let current_media =
                    selected_media_signal();

                if let Some(media) =
                    current_media.clone()
                {
                    let new_value =
                        !media.favorite;

                    let _ =
                        sqlite::set_favorite(
                            &pool,
                            &path,
                            new_value,
                        )
                        .await;

                    let mut updated =
                        media.clone();

                    updated.favorite =
                        new_value;

                    selected_media_signal
                        .set(Some(updated));

                    let current =
                        refresh();

                    refresh.set(
                        current + 1
                    );
                }
            });
        }
    });
    let current_tags: Vec<String> =
        media_tags
            .read_unchecked()
            .as_ref()
            .map(|tags| {
                tags.iter()
                    .map(|tag| tag.name.clone())
                    .collect()
            })
            .unwrap_or_default();

    let pool_for_all_tags =
        pool.clone();

    let pool_for_all_tags =
        pool.clone();



    let all_tags = use_resource(move || {

        let pool =
            pool_for_all_tags.clone();

        let _refresh =
            refresh_counter();

        async move {

            sqlite::get_tags(&pool)
                .await
                .unwrap_or_default()
        }
    });

    let current_all_tags: Vec<String> =
        all_tags
            .read_unchecked()
            .as_ref()
            .map(|tags| {
                tags.iter()
                    .map(|tag| tag.name.clone())
                    .collect()
            })
            .unwrap_or_default();
    
    let add_tag =
        Callback::new({

            let pool =
                pool.clone();

            let mut refresh =
                refresh_counter;

            let selected_media_signal =
                selected_media;

            move |tag_name: String| {

                let pool =
                    pool.clone();

                let selected =
                    selected_media_signal();

                spawn(async move {

                    if let Some(media) = selected {

                        let _ =
                            sqlite::assign_tag(
                                &pool,
                                &media.path,
                                &tag_name,
                            )
                            .await;

                        let current =
                            refresh();

                        refresh.set(
                            current + 1
                        );
                    }
                });
            }
        });
       
    let assign_tag =
        Callback::new({

            let pool =
                pool.clone();

            let mut refresh =
                refresh_counter;

            let selected_media_signal =
                selected_media;

            move |tag_name: String| {

                let pool =
                    pool.clone();

                let selected =
                    selected_media_signal();

                spawn(async move {

                    if let Some(media) = selected {

                        let _ =
                            sqlite::assign_tag(
                                &pool,
                                &media.path,
                                &tag_name,
                            )
                            .await;

                        let current =
                            refresh();

                        refresh.set(
                            current + 1
                        );
                    }
                });
            }
        });

    let remove_tag =
        Callback::new({

            let pool =
                pool.clone();

            let mut refresh =
                refresh_counter;

            let selected_media_signal =
                selected_media;

            move |tag_name: String| {

                let pool =
                    pool.clone();

                let selected =
                    selected_media_signal();

                spawn(async move {

                    if let Some(media) = selected {

                        let _ =
                            sqlite::unassign_tag(
                                &pool,
                                &media.path,
                                &tag_name,
                            )
                            .await;

                        let current =
                            refresh();

                        refresh.set(
                            current + 1
                        );
                    }
                });
            }
        });
    let pool_for_ai =
        pool.clone();

    let ai_analysis = use_resource(move || {

        let pool =
            pool_for_ai.clone();

        let selected =
            selected_media();

        let _refresh =
            refresh_counter();

        async move {

            match selected {

                Some(media) => {

                    match sqlite::get_media_id(
                        &pool,
                        &media.path,
                    )
                    .await
                    {
                        Ok(Some(media_id)) => {

                            sqlite::get_latest_analysis(
                                &pool,
                                media_id,
                                "image_description",
                            )
                            .await
                            .unwrap_or(None)
                        }

                        _ => None,
                    }
                }

                None => None,
            }
        }
    });

    let current_analysis =
        ai_analysis
            .read_unchecked()
            .as_ref()
            .cloned()
            .unwrap_or(None);

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

                div {
                    style: "
        display:flex;
        gap:10px;
        margin-bottom:16px;
    ",

                    button {
                        onclick: {
                            let pool = pool_for_rescan.clone();

                            move |_| {

                                if let Some(folder) =
                                    rfd::FileDialog::new()
                                        .pick_folder()
                                {
                                    let folder =
                                        folder.to_string_lossy().to_string();

                                    let pool = pool.clone();

                                    spawn(async move {

                                        if let Err(e) =
                                            sqlite::add_folder(
                                                &pool,
                                                &folder,
                                            )
                                            .await
                                        {
                                            eprintln!(
                                                "Erreur ajout dossier : {}",
                                                e
                                            );
                                        } else {
                                            println!(
                                                "Dossier ajouté : {}",
                                                folder
                                            );
                                            let _ =
                                                library_service::rescan_library(
                                                    &pool,
                                                    &folder,
                                                    |_current, _total| {},
                                                )
                                                .await;
                                        }
                                    });
                                }
                            }
                        },

                        "📁 Ajouter un dossier"
                    }

                    button {
                        style: "
            padding:10px;
            cursor:pointer;
        ",

                        onclick: {
                            let pool = pool_for_rescan.clone();

                            let mut refresh_handle =
                                refresh_counter;

                            let mut progress_handle =
                                scan_progress;

                            let mut scan_handle =
                                is_scanning;

                            move |_| {

                                scan_handle.set(true);
                                progress_handle.set((0, 1));

                                let pool = pool.clone();

                                spawn(async move {

                                    let result =
                                        library_service::rescan_all_folders(
                                            &pool,
                                            move |current, total| {
                                                progress_handle
                                                    .set((current, total));
                                            }
                                        )
                                        .await;

                                    if let Err(e) = result {
                                        eprintln!(
                                            "Erreur rescan : {}",
                                            e
                                        );
                                    }

                                    scan_handle.set(false);

                                    let current = refresh_handle();
                                    refresh_handle.set(current + 1);
                                });
                            }
                        },

                        "🔄 Rescanner"
                    }
                }

                                            if is_scanning() {

                                                div {
                                                    style: "
                                                        margin-top:10px;
                                                        margin-bottom:20px;
                                                        padding:10px;
                                                        border:1px solid #ccc;
                                                        border-radius:8px;
                                                    ",

                                                    p {
                                                        "🔄 Scan en cours..."
                                                    }

                                                    p {
                                                        "{scan_progress().0} / {scan_progress().1}"
                                                    }

                                                    progress {
                                                        max: "{scan_progress().1}",
                                                        value: "{scan_progress().0}",

                                                        style: "
                                                        width:100%;
                                                        height:20px;
                                                    ",
                                                    }
                                                }
                                            }
                                          if let Some(tags) =
                                                &*tags_panel.read_unchecked()
                                            {
                                                TagsPanel {

                                                    tags: tags
                                                        .iter()
                                                        .map(|tag| TagViewModel {

                                                            name:
                                                                tag.name.clone(),

                                                            count:
                                                                tag.count,
                                                        })
                                                        .collect(),

                                                    selected_tag:
                                                        selected_tag(),

                                                    on_select:
                                                        select_tag,

                                                    on_clear:
                                                        clear_tag_filter,
                                                }
                                            }
                                            p { "Médias présents dans SQLite :" }
                                            if let Some(folder_list)
                                                = &*folders.read_unchecked()
                                            {
                                                FoldersPanel {

                                                    folders: folder_list
                                                        .iter()
                                                        .map(|f| FolderViewModel {

                                                            path:
                                                                f.path.clone(),

                                                            enabled:
                                                                f.enabled,

                                                            media_count:
                                                                f.media_count,
                                                        })
                                                        .collect(),

                                                    on_toggle:
                                                        toggle_folder,

                                                    on_delete:
                                                        delete_folder,
                                                }

                                            }
                                            select {

                                                onchange: move |evt| {
                                                    sort_mode.set(
                                                        evt.value()
                                                    );
                                                },

                                                option {
                                                    value: "name",
                                                    "Nom"
                                                }

                                                option {
                                                    value: "favorite",
                                                    "Favoris"
                                                }

                                                option {
                                                    value: "size",
                                                    "Taille"
                                                }

                                                option {
                                                    value: "duration",
                                                    "Durée"
                                                }

                                                option {
                                                    value: "extension",
                                                    "Extension"
                                                }
                                            }
                                            input {
                                                r#type: "text",

                                                placeholder: "🔍 Rechercher...",

                                                value: "{search_text()}",

                                                oninput: move |evt| {
                                                    search_text.set(evt.value());
                                                },

                                                style: "
                                                    width:100%;
                                                    padding:8px;
                                                    margin-bottom:12px;
                                                ",
                                            }
                                            label {

                                                input {
                                                    r#type: "checkbox",

                                                    checked: show_favorites(),

                                                    onchange: move |_| {

                                                        show_favorites.set(
                                                            !show_favorites()
                                                        );
                                                    },
                                                }

                                                " Afficher uniquement les favoris"
                                            }
                                            
                                            if !medias_sorted.is_empty() {
                                                
                                                MediaGrid {

                                                    medias:
                                                        medias_sorted.clone(),

                                                    tagged_paths:
                                                        tagged_paths.clone(),

                                                    search_text:
                                                        search_text(),

                                                    selected_tag:
                                                        selected_tag(),

                                                    favorites_only:
                                                        show_favorites(),

                                                    on_select:
                                                        select_media,
                                                }

                                            } else {

                                                p {
                                                    "Chargement..."
                                                }
                                            }
                                        }

                                        // Panneau de détails
                                        DetailsPanel {

                                             media:
                                                selected_media(),

                                            media_tags:
                                                current_tags,

                                            all_tags:
                                                current_all_tags,

                                            on_toggle_favorite:
                                                toggle_favorite,

                                            on_add_tag:
                                                add_tag,

                                            on_assign_tag:
                                                assign_tag,

                                            on_remove_tag:
                                                remove_tag,
                                        }
                                        



        }
    }
}

