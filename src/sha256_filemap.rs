use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::path::PathBuf;

pub struct SHA256FileMap {
    hash_to_files: HashMap<String, Vec<PathBuf>>,
}

impl SHA256FileMap {
    pub fn new() -> Self {
        SHA256FileMap {
            hash_to_files: HashMap::new(),
        }
    }

    pub fn upsert_hash(&mut self, filehash: String, filepath: PathBuf) {
        self.hash_to_files
            .entry(filehash.clone())
            .or_default()
            .push(filepath);
    }

    pub fn isolate_duplicates(&mut self) {
        self.hash_to_files.retain(|_, files| files.len() > 1);
    }

    pub fn empty(&self) -> bool {
        self.hash_to_files.len() < 1
    }

    pub fn iter(&self) -> Iter<String, Vec<PathBuf>> {
        self.hash_to_files.iter()
    }
}
