use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    pub code: String,
    pub created_at: DateTime<Utc>,
}
