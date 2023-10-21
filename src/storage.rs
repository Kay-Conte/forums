use std::{sync::{Arc, Mutex, RwLock}, path::Path};

use foxhole::type_cache::TypeCacheKey;
use rusqlite::Connection;

type Shared<T> = Arc<RwLock<T>>;

fn shared<T>(t: T) -> Arc<RwLock<T>> {
    Arc::new(RwLock::new(t))
}

pub struct Counter(usize);

impl Counter {
    pub fn new() -> Shared<Self> {
        shared(Self(0))
    }
    
    pub fn next(&mut self) -> usize {
        self.0 += 1;

        self.0
    }
}

impl TypeCacheKey for Counter {
    type Value = Shared<Counter>;
}

pub struct Database;

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Arc<Mutex<Connection>> {
        let conn = Connection::open(path).unwrap();

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT
            )", ());

        Arc::new(Mutex::new(conn))
    }
}

impl TypeCacheKey for Database {
    type Value = Arc<Mutex<Connection>>;
}
