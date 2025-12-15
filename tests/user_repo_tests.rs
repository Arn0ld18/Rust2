use std::borrow::Cow;
use std::collections::HashMap;

use lab3::*;

struct MemoryStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V> MemoryStorage<K, V> {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl<K: Eq + std::hash::Hash + Copy, V> Storage<K, V> for MemoryStorage<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.data.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}

fn sample_user() -> User {
    User {
        id: 1,
        email: Cow::from("test@gmail.com"),
        activated: true,
    }
}

#[test]
fn test_static_repository() {
    let storage = MemoryStorage::<u64, User>::new();
    let mut repo = UserRepositoryStatic::new(storage);

    let user = sample_user();
    repo.add_user(user.clone());

    assert_eq!(repo.get_user(1), Some(&user));

    let updated = User {
        id: 1,
        email: Cow::from("updatedEMAIL@gmail.com"),
        activated: false,
    };
    repo.update_user(updated.clone());

    assert_eq!(repo.get_user(1), Some(&updated));

    let removed = repo.remove_user(1);
    assert_eq!(removed, Some(updated));
    assert!(repo.get_user(1).is_none());
}

#[test]
fn test_dynamic_repository() {
    let storage = MemoryStorage::<u64, User>::new();
    let mut repo = UserRepositoryDynamic::new(Box::new(storage));

    let user = sample_user();
    repo.add_user(user.clone());

    assert_eq!(repo.get_user(1), Some(&user));

    let updated = User {
        id: 1,
        email: Cow::from("updatedEMAIL@gmail.com"),
        activated: false,
    };
    repo.update_user(updated.clone());

    assert_eq!(repo.get_user(1), Some(&updated));

    let removed = repo.remove_user(1);
    assert_eq!(removed, Some(updated));
    assert!(repo.get_user(1).is_none());
}
