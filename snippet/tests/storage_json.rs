use tempfile::tempdir;
use std::collections::HashMap;
use snippet::storage::{JsonStorage, SnippetStorage};
use snippet::models::Snippet;
use chrono::Utc;

#[test]
fn json_storage_save_load() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("x.json");
    let storage = JsonStorage::new(path.to_string_lossy().into());

    let mut m = HashMap::new();
    m.insert("a".into(), Snippet {
        code: "1+1".into(),
        created_at: Utc::now(),
    });

    storage.save_all(&m).unwrap();

    let loaded = storage.load_all().unwrap();
    assert_eq!(loaded["a"].code, "1+1");
}
