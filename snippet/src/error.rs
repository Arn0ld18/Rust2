use rusqlite;

/// Error type used throughout the snippet application.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    /// Error from the standard IO library (file operations, stdin, stdout).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error related to JSON serialization or deserialization.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error returned by the SQLite database driver.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Error that occurs when parsing a date string into a `chrono` type.
    #[error("Date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    /// Error originating from HTTP requests performed by `reqwest`.
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Returned when trying to read or delete a snippet that does not exist.
    #[error("Snippet '{0}' not found")]
    NotFound(String),

    /// Indicates that the storage backend configuration string was invalid.
    #[error("Invalid storage configuration: {0}")]
    InvalidStorage(String),
}
