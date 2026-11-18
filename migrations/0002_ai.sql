-- Table des tags

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Relation N:N médias <-> tags

CREATE TABLE IF NOT EXISTS media_tags (
    media_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,

    PRIMARY KEY (media_id, tag_id),

    FOREIGN KEY (media_id)
        REFERENCES media(id)
        ON DELETE CASCADE,

    FOREIGN KEY (tag_id)
        REFERENCES tags(id)
        ON DELETE CASCADE
);

-- Index pour accélérer les recherches

CREATE INDEX IF NOT EXISTS idx_tags_name
ON tags(name);

CREATE INDEX IF NOT EXISTS idx_media_tags_media
ON media_tags(media_id);

CREATE INDEX IF NOT EXISTS idx_media_tags_tag
ON media_tags(tag_id);