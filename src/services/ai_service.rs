use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::Duration;
use crate::database::sqlite;

use sqlx::SqlitePool;
use std::process::Command;
use crate::services::storage_service;
const LM_STUDIO_API: &str =
    "http://127.0.0.1:1234/v1/chat/completions";

const MODEL_NAME: &str =
    "qwen2.5-vl";

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: serde_json::Value,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysisResult {
    pub description: String,
    pub tags: Vec<String>,
}

impl AiAnalysisResult {

    pub fn tags_as_csv(&self) -> String {
        self.tags.join(", ")
    }

    pub fn tags_as_json(&self) -> String {
        serde_json::to_string(&self.tags)
            .unwrap_or_default()
    }
}

fn extract_json(
    content: &str,
) -> &str {

    if let Some(start) =
        content.find('{')
    {
        if let Some(end) =
            content.rfind('}')
        {
            return &content[start..=end];
        }
    }

    content
}

pub async fn analyze_image(
    image_path: &str,
) -> Result<AiAnalysisResult> {

    if !Path::new(image_path).exists() {
        bail!(
            "Image introuvable : {}",
            image_path
        );
    }

    let image_base64 =
        image_to_base64(
            image_path
        )?;

    let prompt = r#"
Analyse l'image.

Retourne uniquement un JSON valide.

{
  "description": "...",
  "tags": [
    "...",
    "..."
  ]
}

Description :
- français
- une phrase concise

Tags :
- français
- entre 5 et 15 tags
- minuscules
- sans doublons

Ne retourne aucun texte hors JSON.
"#;

    let body = json!({

        "model": MODEL_NAME,

        "temperature": 0.2,

        "messages": [

            {
                "role": "user",

                "content": [

                    {
                        "type": "text",
                        "text": prompt
                    },

                    {
                        "type": "image_url",

                        "image_url": {
                            "url": format!(
                                "data:image/jpeg;base64,{}",
                                image_base64
                            )
                        }
                    }
                ]
            }
        ]
    });

    let client =
        Client::builder()
            .timeout(
                Duration::from_secs(120)
            )
            .build()?;

    let response =
        client
            .post(LM_STUDIO_API)
            .json(&body)
            .send()
            .await?;

    let value: serde_json::Value =
        response.json().await?;

    let content =
        value["choices"][0]
            ["message"]
            ["content"]
            .as_str()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Réponse LM Studio invalide"
                )
            })?;

    println!(
        "LM Studio : {}",
        content
    );

    let clean_json =
        extract_json(content);

    let analysis:
        AiAnalysisResult =
            serde_json::from_str(
                clean_json
            )?;

    Ok(analysis)
}

pub async fn analyze_video(
    video_path: &str,
) -> Result<AiAnalysisResult> {

    let frames =
        extract_video_frames(
            video_path
        )?;

    if frames.is_empty() {

        bail!(
            "Aucune frame extraite"
        );
    }

    let mut descriptions =
        Vec::new();

    let mut all_tags =
        Vec::new();

    for frame in
        frames.iter().take(5)
    {

        match analyze_image(frame)
            .await
        {
            Ok(result) => {

                descriptions.push(
                    result.description
                );

                all_tags.extend(
                    result.tags
                );
            }

            Err(e) => {

                eprintln!(
                    "Erreur frame: {}",
                    e
                );
            }
        }
    }

    all_tags.sort();

    all_tags.dedup();

    Ok(
        AiAnalysisResult {

            description:
                descriptions.join(
                    " "
                ),

            tags: all_tags,
        }
    )
}

pub async fn analyze_media(
    media_path: &str,
    media_type: &str,
) -> Result<AiAnalysisResult> {

    match media_type {

        "image" => {
            analyze_image(media_path)
                .await
        }

        "video" => {
            analyze_video(media_path)
                .await
        }

        _ => {

            Ok(
                AiAnalysisResult {

                    description:
                        "Type multimédia non supporté."
                            .to_string(),

                    tags: vec![
                        "unknown".to_string(),
                    ],
                }
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct StoredAiAnalysis {

    pub media_id: i64,

    pub model_name: String,

    pub analysis_type: String,

    pub result_text: String,

    pub raw_json: Option<String>,
}
pub async fn analyze_and_store(
    pool: &SqlitePool,

    media_path: &str,

    media_type: &str,
) -> anyhow::Result<()> {

    let analysis =
        analyze_media(
            media_path,
            media_type,
        )
        .await?;
    sqlite::apply_ai_tags(
        pool,
        media_path,
        &analysis.tags,
    )
    .await?;
    let media_id =
        sqlite::get_media_id(
            pool,
            media_path,
        )
        .await?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Media introuvable"
            )
        })?;

    let raw_json =
        serde_json::to_string(
            &analysis
        )?;

    sqlite::save_ai_analysis(
        pool,

        media_id,

        "qwen2.5-vl",

        "image_description",

        &analysis.description,

        Some(&raw_json),
    )
    .await?;

    Ok(())
}
fn build_prompt() -> String {

        r#"
    Analyse cette image.

    Réponds UNIQUEMENT en JSON valide.

    Format attendu :

    {
    "description": "...",
    "tags": [
        "tag1",
        "tag2",
        "tag3"
    ]
    }

    Les tags doivent être courts.
    5 à 15 tags maximum.
    Pas de texte hors JSON.
    "#
        .to_string()
}
fn image_to_base64(
    image_path: &str,
) -> anyhow::Result<String> {

    let bytes =
        fs::read(image_path)?;

    Ok(
        general_purpose::STANDARD
            .encode(bytes)
    )
}
fn extract_video_frames(
    video_path: &str,
) -> Result<Vec<String>> {

    let frames_dir =
        storage_service::video_frames_dir(
            video_path
        );

    std::fs::create_dir_all(
        &frames_dir
    )?;

    let output_pattern =
        frames_dir
            .join("frame_%03d.jpg");

    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            video_path,
            "-vf",
            "fps=1/10",
        ])
        .arg(&output_pattern)
        .status()?;

    if !status.success() {
        bail!(
            "Extraction vidéo impossible"
        );
    }

    let mut frames = Vec::new();

    for entry in
        std::fs::read_dir(
            &frames_dir
        )?
    {
        let entry = entry?;

        frames.push(
            entry
                .path()
                .to_string_lossy()
                .to_string()
        );
    }

    frames.sort();

    Ok(frames)
}
pub fn normalize_tag(
    tag: &str,
) -> String {

    let mut tag =
        tag.trim()
            .to_lowercase();

    tag = tag
        .replace('à', "a")
        .replace('â', "a")
        .replace('ä', "a")
        .replace('é', "e")
        .replace('è', "e")
        .replace('ê', "e")
        .replace('ë', "e")
        .replace('î', "i")
        .replace('ï', "i")
        .replace('ô', "o")
        .replace('ö', "o")
        .replace('ù', "u")
        .replace('û', "u")
        .replace('ü', "u")
        .replace('ç', "c");

    tag
}
pub fn is_allowed_tag(
    tag: &str,
) -> bool {

    tag.chars().all(|c| {

        c.is_ascii_alphabetic()
            ||
        c.is_ascii_whitespace()
            ||
        c == '-'
    })
}