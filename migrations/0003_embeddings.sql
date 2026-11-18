-- Embeddings générés par l'IA locale
-- (BGE-M3, Qwen Embeddings, Nomic Embed, etc.)

CREATE TABLE IF NOT EXISTS embeddings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    media_id INTEGER NOT NULL UNIQUE,

    model_name TEXT NOT NULL,

    dimensions INTEGER NOT NULL,

    vector BLOB NOT NULL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (media_id)
        REFERENCES media(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_embeddings_media
ON embeddings(media_id);

CREATE INDEX IF NOT EXISTS idx_embeddings_model
ON embeddings(model_name);