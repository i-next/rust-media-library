#[derive(Debug, Clone)]
pub struct WatchedFolder {
    pub path: String,
    pub enabled: bool,
    pub media_count: i64,
}
