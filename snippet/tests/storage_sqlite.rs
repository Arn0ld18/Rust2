use tempfile::tempdir;
use std::collections::HashMap;
use snippet::storage::{SqliteStorage, SnippetStorage};
use snippet::models::Snippet;
use chrono::Utc;

#[test]
fn sqlite_storage_save_load() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("db.sqlite");

    let storage = SqliteStorage::new(path.to_string_lossy().into()).unwrap();

    let mut m = HashMap::new();
    m.insert("b".into(), Snippet {
        code: "println!();".into(),
        created_at: Utc::now(),
    });

    storage.save_all(&m).unwrap();
    let loaded = storage.load_all().unwrap();

    assert_eq!(loaded["b"].code, "println!();");
}
