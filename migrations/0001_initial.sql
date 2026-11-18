

CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS media (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_id INTEGER,

    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    extension TEXT,

    media_type TEXT NOT NULL,

    file_size INTEGER,

    width INTEGER,
    height INTEGER,

    duration REAL,

    thumbnail_path TEXT,

    description TEXT,

    analyzed INTEGER NOT NULL DEFAULT 0,
    favorite INTEGER NOT NULL DEFAULT 0,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    indexed_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(folder_id)
        REFERENCES folders(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_path
ON media(path);

CREATE INDEX IF NOT EXISTS idx_media_filename
ON media(filename);

CREATE INDEX IF NOT EXISTS idx_media_type
ON media(media_type);

CREATE INDEX IF NOT EXISTS idx_folder_path
ON folders(path);

CREATE VIRTUAL TABLE IF NOT EXISTS media_search
USING fts5(
    filename,
    description,
    content='media',
    content_rowid='id'
);

