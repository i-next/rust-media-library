// src/models/tag.rs

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct TagCount {
    pub name: String,
    pub count: i64,
}