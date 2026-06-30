use std::collections::HashMap;
use std::path::PathBuf;

pub struct SHA256FileMap {
    pub hashes: HashMap<String, Vec<PathBuf>>,
}

impl SHA256FileMap {
    pub fn new() -> Self {
        SHA256FileMap {
            hashes: HashMap::new(),
        }
    }

    pub fn upsert_hash(&mut self, filehash: String, filepath: PathBuf) {
        self.hashes
            .entry(filehash.clone())
            .or_default()
            .push(filepath);
    }

    pub fn isolate_duplicates(&mut self) {
        self.hashes.retain(|_, files| files.len() > 1);
    }

    pub fn empty(&self) -> bool {
        self.hashes.len() < 1
    }
}
