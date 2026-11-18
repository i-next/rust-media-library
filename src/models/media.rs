#[derive(Debug, Clone)]
pub struct MediaFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub media_type: String,
    pub thumbnail_path: Option<String>,
    pub thumbnail_data: Option<String>,
}