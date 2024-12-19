use dashmap::DashMap;
use std::sync::Arc;
use crate::models::SearchResult;

#[derive(Clone)]
pub struct QueryCache {
    inner: Arc<DashMap<String, Vec<SearchResult>>>,
}

impl QueryCache {
    pub fn new() -> Self {
        QueryCache {
            inner: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<SearchResult>> {
        self.inner.get(key).map(|v| v.clone())
    }

    pub fn set(&self, key: String, value: Vec<SearchResult>) {
        self.inner.insert(key, value);
    }
}
