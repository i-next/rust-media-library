use dioxus::prelude::*;

use crate::services::library_service;
use crate::{app_state::AppState, database::sqlite};

use crate::ui::components::statistics_panel::StatisticsPanel;
use crate::ui::components::folders_panel::{
    FolderViewModel,
    FoldersPanel,
};
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
    let mut search_text = use_signal(String::new);
    use crate::models::media::MediaFile;

    let mut selected_media = use_signal(|| None::<MediaFile>);
    let pool = state.pool.clone();
    let pool_for_tags = pool.clone();

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
    let mut new_tag =
        use_signal(|| String::new());
    let mut show_favorites =
    use_signal(|| false);
    let mut sort_mode =
        use_signal(|| "name".to_string());

    let mut selected_tag =
        use_signal(|| None::<String>);
    let mut selected_existing_tag =
        use_signal(|| String::new());
    let favorites_only =
    show_favorites();
    let selected = selected_media();
    let media_tags = use_resource(move || {

        let pool = pool_for_tags.clone();

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

    let mut medias_sorted = if let Some(list) =
        &*medias.read_unchecked()
    {
        list.clone()
    } else {
        Vec::new()
    };

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
    let pool_for_stats =
        pool.clone();

        let stats = use_resource(move || {

            let pool =
                pool_for_stats.clone();

            let _refresh =
                refresh_counter();

            async move {

                sqlite::get_library_stats(&pool)
                    .await
                    .ok()
            }
        });
    let pool_for_all_tags = pool.clone();

        let all_tags = use_resource(move || {

            let pool = pool_for_all_tags.clone();

            let _refresh =
                refresh_counter();

            async move {

                sqlite::get_tags(&pool)
                    .await
                    .unwrap_or_default()
            }
        });
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
                                                div {
                                                    style: "
                                                        margin-bottom:20px;
                                                        padding:10px;
                                                        border:1px solid #ccc;
                                                        border-radius:8px;
                                                    ",
                                                if let Some(Some(stats)) =
                                                    &*stats.read_unchecked()
                                                {
                                                    StatisticsPanel {

                                                        total_media:
                                                            stats.total_media,

                                                        total_images:
                                                            stats.total_images,

                                                        total_videos:
                                                            stats.total_videos,

                                                        total_favorites:
                                                            stats.total_favorites,

                                                        total_tags:
                                                            stats.total_tags,

                                                        total_size:
                                                            stats.total_size,
                                                    }
                                                }
                                                    h3 {
                                                        "🏷️ Tags"
                                                    }

                                                    for tag in tags {

                                                        div {

                                                            style: "
                                                                cursor:pointer;
                                                                padding:4px;
                                                            ",

                                                            onclick: {
                                                                let tag_name =
                                                                    tag.name.clone();
                                                                println!(
                                                                        "Tag sélectionné = {:?}",
                                                                        selected_tag()
                                                                    );
                                                                    println!(
    "Paths trouvés = {}",
    tagged_paths.len()
);
                                                                move |_| {
                                                                   
                                                                    selected_tag.set(
                                                                        Some(tag_name.clone())
                                                                    );
                                                                }
                                                            },

                                                            "{tag.name} ({tag.count})"
                                                        }
                                                    }

                                                    button {

                                                        onclick: move |_| {
                                                            selected_tag.set(None);
                                                        },

                                                        "Effacer filtre"
                                                    }
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
                                                
                                                div {
                                                    style: "
                                                        display:flex;
                                                        flex-wrap:wrap;
                                                        gap:16px;
                                                    ",
                                                
                                                    for media in medias_sorted.iter().filter(|media| {

                                                        let search =
                                                            search_text()
                                                                .to_lowercase();

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
                                                            tagged_paths
                                                                .contains(&media.path);

                                                        let matches_search =
                                                            matches_name
                                                            || matches_tag;

                                                        let matches_favorite =
                                                            !show_favorites()
                                                            || media.favorite;

                                                        let matches_selected_tag =

                                                            selected_tag().is_none()

                                                            ||

                                                            tagged_paths.contains(
                                                                &media.path
                                                            );
                                                        matches_search
                                                            &&
                                                        matches_favorite
                                                            &&
                                                        matches_selected_tag


                                                    }) {

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
                                                                let media_clone = media.clone();

                                                                move |_| {
                                                                    selected_media.set(Some(media_clone.clone()));
                                                                }
                                                            },
                                                            ondoubleclick: {
                                                                let path = media.path.clone();

                                                                move |_| {
                                                                    let _ = open::that(path.clone());
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
                                        if media.favorite {
                                            "⭐ "
                                        }

                                        "{media.filename}"
                                    }
                                    button {

                                        onclick: {
                                            let path = media.path.clone();
                                            let pool = state.pool.clone();
                                            let mut selected_handle = selected_media;
                                            let mut refresh_handle =
                                                refresh_counter;

                                            let favorite =
                                                media.favorite;

                                            move |_| {

                                                let path = path.clone();
                                                let pool = pool.clone();

                                                spawn(async move {

                                                    if let Err(e) =
                                                        sqlite::set_favorite(
                                                            &pool,
                                                            &path,
                                                            !favorite,
                                                        )
                                                        .await
                                                    {
                                                        eprintln!(
                                                            "Erreur favori : {}",
                                                            e
                                                        );
                                                        return;
                                                    }
                                                    if let Some(mut current_media) =
                                                        selected_handle()
                                                    {
                                                        current_media.favorite =
                                                            !favorite;

                                                        selected_handle.set(
                                                            Some(current_media)
                                                        );
                                                    }
                                                    let current =
                                                        refresh_handle();

                                                    refresh_handle.set(
                                                        current + 1
                                                    );
                                                });
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
                                    hr {}

                                    button {

                                        onclick: {

                                            let pool =
                                                pool.clone();

                                            let path =
                                                media.path.clone();

                                            let media_type =
                                                media.media_type.clone();

                                            let mut refresh =
                                                refresh_counter;

                                            move |_| {

                                                let pool =
                                                    pool.clone();

                                                let path =
                                                    path.clone();

                                                let media_type =
                                                    media_type.clone();

                                                spawn(async move {

                                                    if let Err(e) =
                                                        crate::services::ai_service
                                                            ::analyze_and_store(
                                                                &pool,
                                                                &path,
                                                                &media_type,
                                                            )
                                                            .await
                                                    {
                                                        eprintln!(
                                                            "Erreur IA: {}",
                                                            e
                                                        );
                                                    }

                                                    let current =
                                                        refresh();

                                                    refresh.set(
                                                        current + 1
                                                    );
                                                });
                                            }
                                        },

                                        "🤖 Analyser"
                                    }

                                    hr {}

                                    h3 {
                                        "🤖 Analyse IA"
                                    }

                                    if let Some(Some(description)) =
                                        &*ai_analysis.read_unchecked()
                                    {
                                        div {

                                            style: "
                                                margin-bottom:16px;
                                                padding:10px;
                                                border:1px solid #ccc;
                                                border-radius:8px;
                                                background:#f8f8f8;
                                            ",

                                            p {
                                                "{description}"
                                            }
                                        }
                                    }
                                    else {

                                        p {
                                            style: "color:#888;",
                                            "Aucune analyse IA disponible."
                                        }
                                    }

                                    hr {}
                                    
                                    h3 {
                                        "🏷️ Tags"
                                    }

                                    
                                    if let Some(tags) =
                                        &*media_tags.read_unchecked()
                                    {
                                        div {
                                            style: "
                                                display:flex;
                                                flex-wrap:wrap;
                                                gap:6px;
                                                margin-bottom:10px;
                                            ",

                                            for tag in tags {

                                                div {
                                                    style: "
                                                        background:#3b82f6;
                                                        color:white;
                                                        padding:4px 8px;
                                                        border-radius:12px;
                                                        display:flex;
                                                        align-items:center;
                                                        gap:6px;
                                                    ",

                                                    "{tag.name}"

                                                    button {

                                                        style: "
                                                            border:none;
                                                            cursor:pointer;
                                                            background:transparent;
                                                            color:white;
                                                        ",

                                                        onclick: {
                                                            let pool = pool.clone();

                                                            let path =
                                                                media.path.clone();

                                                            let tag_name =
                                                                tag.name.clone();

                                                            let mut refresh =
                                                                refresh_counter;

                                                            move |_| {

                                                                let pool =
                                                                    pool.clone();

                                                                let path =
                                                                    path.clone();

                                                                let tag_name =
                                                                    tag_name.clone();

                                                                spawn(async move {

                                                                    let _ =
                                                                        sqlite::unassign_tag(
                                                                            &pool,
                                                                            &path,
                                                                            &tag_name,
                                                                        )
                                                                        .await;

                                                                    let current =
                                                                        refresh();

                                                                    refresh.set(
                                                                        current + 1
                                                                    );
                                                                });
                                                            }
                                                        },

                                                        "✕"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    

                                    div {

                                    /* input {

                                        r#type: "text",

                                        placeholder: "Nouveau tag",
                                        
                                        value: "{new_tag()}",

                                        oninput: move |evt| {

                                            new_tag.set(
                                                evt.value()
                                            );
                                        },
                                    }
                                    if let Some(tags) =
                                        &*all_tags.read_unchecked()
                                    {
                                        select {

                                            onchange: move |evt| {

                                                selected_existing_tag
                                                    .set(evt.value());
                                            },

                                            option {
                                                value: "",
                                                "-- Tag existant --"
                                            }

                                            for tag in tags {

                                                option {
                                                    value: "{tag.name}",
                                                    "{tag.name}"
                                                }
                                            }
                                        }
                                    } */

                                    div {

    input {

        r#type: "text",

        placeholder: "Nouveau tag",

        value: "{new_tag()}",

        oninput: move |evt| {
            new_tag.set(
                evt.value()
            );
        },
    }

    button {

        onclick: {

            let pool = pool.clone();

            let path =
                media.path.clone();

            let mut refresh =
                refresh_counter;

            let mut tag_signal =
                new_tag;

            move |_| {

                let pool =
                    pool.clone();

                let path =
                    path.clone();

                let tag_name =
                    tag_signal();

                spawn(async move {

                    if !tag_name
                        .trim()
                        .is_empty()
                    {
                        let _ =
                            sqlite::assign_tag(
                                &pool,
                                &path,
                                &tag_name,
                            )
                            .await;
                    }

                    tag_signal.set(
                        String::new()
                    );

                    let current =
                        refresh();

                    refresh.set(
                        current + 1
                    );
                });
            }
        },

        "➕ Créer le tag"
        }

        if let Some(tags) =
            &*all_tags.read_unchecked()
        {
            select {

                onchange: move |evt| {

                    selected_existing_tag
                        .set(evt.value());
                },

                option {
                    value: "",
                    "-- Tag existant --"
                }

                for tag in tags {

                    option {
                        value: "{tag.name}",
                        "{tag.name}"
                    }
                }
            }
        }

        button {

            onclick: {

                let pool = pool.clone();

                let path =
                    media.path.clone();

                let mut refresh =
                    refresh_counter;

                let selected_tag_signal =
                    selected_existing_tag;

                move |_| {

                    let pool =
                        pool.clone();

                    let path =
                        path.clone();

                    let tag_name =
                        selected_tag_signal();

                    spawn(async move {

                        println!(
                            "Tag sélectionné = {}",
                            tag_name
                        );

                        if !tag_name
                            .trim()
                            .is_empty()
                        {
                            let _ =
                                sqlite::assign_tag(
                                    &pool,
                                    &path,
                                    &tag_name,
                                )
                                .await;
                        }

                        let current =
                            refresh();

                        refresh.set(
                            current + 1
                        );
                    });
                }
            },

            "🔗 Associer le tag"
        }
    }

                                    /* button {

                                        onclick: {
                                            let pool = pool.clone();

                                            let path =
                                                media.path.clone();

                                            let mut refresh =
                                                refresh_counter;

                                            let mut tag_signal =
                                                new_tag;

                                            move |_| {

                                                let pool =
                                                    pool.clone();

                                                let path =
                                                    path.clone();

                                                let tag_name =
                                                    tag_signal();

                                                spawn(async move {

                                                    if !tag_name
                                                        .trim()
                                                        .is_empty()
                                                    {
                                                        let _ =
                                                            sqlite::assign_tag(
                                                                &pool,
                                                                &path,
                                                                &tag_name,
                                                            )
                                                            .await;
                                                    }

                                                    tag_signal.set(
                                                        String::new()
                                                    );

                                                    let current =
                                                        refresh();

                                                    refresh.set(
                                                        current + 1
                                                    );
                                                });
                                            }
                                        },

                                       
                                        button {

                                            onclick: {

                                                let pool = pool.clone();

                                                let path =
                                                    media.path.clone();

                                                let mut refresh =
                                                    refresh_counter;

                                                let selected_tag_signal =
                                                    selected_existing_tag;

                                                move |_| {

                                                    let pool =
                                                        pool.clone();

                                                    let path =
                                                        path.clone();

                                                    let tag_name =
                                                        selected_tag_signal();

                                                    spawn(async move {
                                                        println!(
                                                            "Tag sélectionné = {}",
                                                            tag_name
                                                        );
                                                        if !tag_name.is_empty()
                                                        {
                                                            let _ =
                                                                sqlite::assign_tag(
                                                                    &pool,
                                                                    &path,
                                                                    &tag_name,
                                                                )
                                                                .await;
                                                        }

                                                        let current =
                                                            refresh();

                                                        refresh.set(
                                                            current + 1
                                                        );
                                                    });
                                                }
                                            },

                                            "Ajouter - Associer le tag"
                                        }
                                    } */
                                    hr {}
                                }
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
                                    /* button {

                                        onclick: {
                                            let path = media.path.clone();
                                            let pool = state.pool.clone();

                                            move |_| {

                                                let path = path.clone();
                                                let pool = pool.clone();

                                                spawn(async move {

                                                    let _ =
                                                        sqlite::set_favorite(
                                                            &pool,
                                                            &path,
                                                            !media.favorite,
                                                        )
                                                        .await;
                                                });
                                            }
                                        }, 

                                        if media.favorite {
                                            "⭐ Retirer des favoris"
                                        } else {
                                            "☆ Ajouter aux favoris"
                                        }
                                    }*/
                                    p {
                                        strong { "Miniature générée :" }
                                        " "
                                        if media.thumbnail_data.is_some() {
                                            "Oui"
                                        } else {
                                            "Non"
                                        }
                                    }
                                    
                                    if let Some(data_uri) =
                                        &media.thumbnail_data
                                    {
                                        img {
                                            src: "{data_uri}",
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
