// library_stats.rs

#[derive(Debug, Clone)]
pub struct LibraryStats {
    pub total_media: i64,
    pub total_images: i64,
    pub total_videos: i64,
    pub total_favorites: i64,
    pub total_tags: i64,
    pub total_size: i64,
}