use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single saved code snippet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    /// Snippet source code.
    pub code: String,
    /// Timestamp of creation.
    pub created_at: DateTime<Utc>,
}
