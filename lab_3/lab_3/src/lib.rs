use std::borrow::Cow;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

pub struct UserRepositoryStatic<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

pub struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    pub fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}
