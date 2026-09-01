use std::collections::HashMap;
use std::path::PathBuf;

use crate::utils;

#[derive(Debug)]
pub struct Duplicates {
    hashes: HashMap<String, Vec<PathBuf>>,
}

impl Duplicates {
    pub fn new() -> Self {
        Duplicates {
            hashes: HashMap::new(),
        }
    }

    pub fn get_image_file_hashes(mut self, files: Vec<PathBuf>) -> Self {
        for file in files {
            match utils::compute_file_sha256(&file) {
                Ok(hash) => self.hashes.entry(hash).or_default().push(file),
                Err(error) => {
                    eprintln!("Could not get hash for file '{}': {error}", file.display())
                }
            }
        }
        self
    }

    pub fn isolate_duplicate_files(mut self) -> Self {
        self.hashes.retain(|_, files| files.len() > 1);
        self
    }

    pub fn get_duplicate_files_or_none(self) -> Option<HashMap<String, Vec<PathBuf>>> {
        if self.hashes.is_empty() {
            None
        } else {
            Some(self.hashes)
        }
    }
}
