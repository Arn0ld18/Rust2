use crate::models::Snippet;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

// ------------------ STORAGE TRAIT ------------------
pub trait SnippetStorage {
    fn load_all(&self) -> HashMap<String, Snippet>;
    fn save_all(&self, data: &HashMap<String, Snippet>);
    fn delete(&self, name: &str);
}

// ------------------ JSON STORAGE ------------------
pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    pub fn new(path: String) -> Self { Self { path } }
}

impl SnippetStorage for JsonStorage {
    fn load_all(&self) -> HashMap<String, Snippet> {
        if Path::new(&self.path).exists() {
            let data = fs::read_to_string(&self.path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else { HashMap::new() }
    }

    fn save_all(&self, data: &HashMap<String, Snippet>) {
        let json = serde_json::to_string_pretty(data).unwrap();
        fs::write(&self.path, json).unwrap();
    }

    fn delete(&self, name: &str) {
        let mut all = self.load_all();
        all.remove(name);
        self.save_all(&all);
    }
}

// ------------------ SQLITE STORAGE ------------------
pub struct SqliteStorage {
    path: String,
}

impl SqliteStorage {
    pub fn new(path: String) -> Self {
        let storage = Self { path };
        storage.init_db();
        storage
    }

    fn init_db(&self) {
        let conn = Connection::open(&self.path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                name TEXT PRIMARY KEY,
                code TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).unwrap();
    }
}

impl SnippetStorage for SqliteStorage {
    fn load_all(&self) -> HashMap<String, Snippet> {
        let conn = Connection::open(&self.path).unwrap();
        let mut stmt = conn.prepare("SELECT name, code, created_at FROM snippets").unwrap();
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                Snippet {
                    code: row.get(1)?,
                    created_at: row.get::<_, String>(2)?.parse().unwrap(),
                },
            ))
        }).unwrap();

        let mut map = HashMap::new();
        for r in rows { let (name, snippet) = r.unwrap(); map.insert(name, snippet); }
        map
    }

    fn save_all(&self, data: &HashMap<String, Snippet>) {
        let conn = Connection::open(&self.path).unwrap();
        conn.execute("DELETE FROM snippets", []).unwrap();
        for (name, snip) in data {
            conn.execute(
                "INSERT INTO snippets (name, code, created_at) VALUES (?1, ?2, ?3)",
                params![name, snip.code, snip.created_at.to_rfc3339()],
            ).unwrap();
        }
    }

    fn delete(&self, name: &str) {
        let conn = Connection::open(&self.path).unwrap();
        conn.execute("DELETE FROM snippets WHERE name = ?1", [name]).unwrap();
    }
}

// ------------------ STORAGE SELECTOR ------------------
pub fn get_storage() -> Box<dyn SnippetStorage> {
    let env_var = env::var("SNIPPETS_APP_STORAGE").unwrap_or_else(|_| "JSON:snippets.json".to_string());
    let parts: Vec<&str> = env_var.split(':').collect();
    if parts.len() != 2 { panic!("SNIPPETS_APP_STORAGE format must be PROVIDER:/path/file"); }

    match parts[0] {
        "JSON" => Box::new(JsonStorage::new(parts[1].to_string())),
        "SQLITE" => Box::new(SqliteStorage::new(parts[1].to_string())),
        _ => panic!("Unknown storage provider: {}", parts[0]),
    }
}
