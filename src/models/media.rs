#[derive(Debug, Clone, PartialEq)]
pub struct MediaFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub media_type: String,

    pub thumbnail_path: Option<String>,
    pub thumbnail_data: Option<String>,

    pub file_size: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,

    pub duration: Option<f64>,
    pub favorite: bool,

}
