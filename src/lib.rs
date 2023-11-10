use std::collections::HashMap;

pub struct KvStore {
    _store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            _store: HashMap::new(),
        }
    }

    pub fn get(&self, k: String) -> Option<String> {
        self._store.get(&k).cloned()
    }

    pub fn set(&mut self, k: String, v: String) {
        self._store.insert(k, v);
    }

    pub fn remove(&mut self, k: String) {
        self._store.remove(&k);
    }
}
