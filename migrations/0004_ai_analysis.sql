-- Historique des analyses IA

CREATE TABLE IF NOT EXISTS ai_analysis (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    media_id INTEGER NOT NULL,

    model_name TEXT NOT NULL,

    analysis_type TEXT NOT NULL,

    prompt TEXT,

    result_text TEXT NOT NULL,

    raw_json TEXT,

    confidence REAL,

    processing_time_ms INTEGER,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (media_id)
        REFERENCES media(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_ai_analysis_media
ON ai_analysis(media_id);

CREATE INDEX IF NOT EXISTS idx_ai_analysis_model
ON ai_analysis(model_name);

CREATE INDEX IF NOT EXISTS idx_ai_analysis_type
ON ai_analysis(analysis_type);