CREATE VIRTUAL TABLE IF NOT EXISTS media_filename_search
USING fts5(
    path,
    filename,
    extension,
    media_type
);