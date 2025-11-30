use crate::error::AppError;
use crate::models::Snippet;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

// ------------------ STORAGE TRAIT ------------------
/// Storage backend for managing snippets.
pub trait SnippetStorage {
    /// Loads all snippets from storage.
    fn load_all(&self) -> Result<HashMap<String, Snippet>, AppError>;
    /// Saves the entire snippet collection.
    fn save_all(&self, data: &HashMap<String, Snippet>) -> Result<(), AppError>;
    /// Removes a single snippet by name.
    fn delete(&self, name: &str) -> Result<(), AppError>;
}
// ------------------ JSON STORAGE ------------------

/// JSON file–based implementation of [`SnippetStorage`].
pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    /// Creates a new JSON storage with file path.
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl JsonStorage {
    /// Load all data.
    pub fn load_all(&self) -> Result<HashMap<String, Snippet>, AppError> {
        if Path::new(&self.path).exists() {
            let data = fs::read_to_string(&self.path)?;
            let map = serde_json::from_str(&data)?;
            Ok(map)
        } else {
            Ok(HashMap::new())
        }
    }
    /// Save all data.
    pub fn save_all(&self, data: &HashMap<String, Snippet>) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }
    /// Delete by name.
    pub fn delete(&self, name: &str) -> Result<(), AppError> {
        let mut all = self.load_all()?;
        all.remove(name);
        self.save_all(&all)?;
        Ok(())
    }
}
impl SnippetStorage for JsonStorage {
    fn load_all(&self) -> Result<HashMap<String, Snippet>, AppError> {
        if Path::new(&self.path).exists() {
            let data = fs::read_to_string(&self.path)?;
            let map = serde_json::from_str(&data)?;
            Ok(map)
        } else {
            Ok(HashMap::new())
        }
    }

    fn save_all(&self, data: &HashMap<String, Snippet>) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), AppError> {
        let mut all = self.load_all()?;
        all.remove(name);
        self.save_all(&all)?;
        Ok(())
    }
}

// ------------------ SQLITE STORAGE ------------------
/// SQLite-based implementation of [`SnippetStorage`].
pub struct SqliteStorage {
    path: String,
}

impl SqliteStorage {
    /// Creates a new SQLite storage and initializes schema.
    ///
    /// # Errors
    /// Returns `AppError` if the database file cannot be created or schema creation fails.
    pub fn new(path: String) -> Result<Self, AppError> {
        let storage = Self { path };
        storage.init_db()?;
        Ok(storage)
    }

    fn init_db(&self) -> Result<(), AppError> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                name TEXT PRIMARY KEY,
                code TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}
impl SnippetStorage for SqliteStorage {
    fn load_all(&self) -> Result<HashMap<String, Snippet>, AppError> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT name, code, created_at FROM snippets")?;
        let rows = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let code: String = row.get(1)?;
            let created_at_str: String = row.get(2)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                .with_timezone(&Utc);
            Ok((name, Snippet { code, created_at }))
        })?;

        let mut map = HashMap::new();
        for r in rows {
            let (name, snippet) = r?;
            map.insert(name, snippet);
        }
        Ok(map)
    }

    fn save_all(&self, data: &HashMap<String, Snippet>) -> Result<(), AppError> {
        let conn = Connection::open(&self.path)?;
        conn.execute("DELETE FROM snippets", [])?;
        for (name, snip) in data {
            conn.execute(
                "INSERT INTO snippets (name, code, created_at) VALUES (?1, ?2, ?3)",
                params![name, snip.code, snip.created_at.to_rfc3339()],
            )?;
        }
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), AppError> {
        let conn = Connection::open(&self.path)?;
        conn.execute("DELETE FROM snippets WHERE name = ?1", [name])?;
        Ok(())
    }
}

// ------------------ STORAGE SELECTOR ------------------

/// Selects storage backend from `SNIPPETS_APP_STORAGE`.
///
/// Format: `JSON:path` or `SQLITE:path`.
///
/// # Errors
/// Returns `InvalidStorage` if the variable format is wrong or SQLite fails.
pub fn get_storage() -> Result<Box<dyn SnippetStorage>, AppError> {
    let env_var =
        env::var("SNIPPETS_APP_STORAGE").unwrap_or_else(|_| "JSON:snippets.json".to_string());
    let parts: Vec<&str> = env_var.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::InvalidStorage(env_var));
    }

    match parts[0] {
        "JSON" => Ok(Box::new(JsonStorage::new(parts[1].to_string()))),
        "SQLITE" => Ok(Box::new(SqliteStorage::new(parts[1].to_string())?)),
        _ => Err(AppError::InvalidStorage(parts[0].to_string())),
    }
}
