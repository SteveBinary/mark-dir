use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dirmark {
    pub path: PathBuf,
}

impl Dirmark {
    pub fn with_value(value: PathBuf) -> Self {
        Self { path: value }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dirmarks {
    dirmarks: BTreeMap<String, Dirmark>,
}

impl Dirmarks {
    pub fn new() -> Self {
        Self {
            dirmarks: BTreeMap::new(),
        }
    }

    pub fn insert_at(&mut self, key: &str, value: PathBuf) -> Option<Dirmark> {
        self.dirmarks.insert(key.into(), Dirmark::with_value(value))
    }

    pub fn insert(&mut self, key: &[String], value: PathBuf) -> Option<Dirmark> {
        self.insert_at(&key.join("."), value)
    }

    pub fn get_at(&self, key: &str) -> Option<&Dirmark> {
        self.dirmarks.get(key)
    }

    pub fn get(&self, key: &[String]) -> Option<&Dirmark> {
        self.get_at(&key.join("."))
    }

    pub fn get_all(&self) -> &BTreeMap<String, Dirmark> {
        &self.dirmarks
    }

    pub fn delete_at(&mut self, key: &str) -> Option<Dirmark> {
        self.dirmarks.remove(key)
    }

    pub fn delete(&mut self, key: &[String]) -> Option<Dirmark> {
        self.delete_at(&key.join("."))
    }
}
