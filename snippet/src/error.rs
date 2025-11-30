use rusqlite;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Snippet '{0}' not found")]
    NotFound(String),

    #[error("Invalid storage configuration: {0}")]
    InvalidStorage(String),
}

