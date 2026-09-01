use std::path::PathBuf;
use std::fs::read_dir;

use anyhow::Context;

use crate::utils;

#[derive(Debug)]
pub struct ImageFiles {
    files: Vec<PathBuf>,
}

impl ImageFiles {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn locate_all_files(mut self, dir: &PathBuf) -> anyhow::Result<Self> {
        let entries =
            read_dir(dir).context(format!("failed to read directory: {}", dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                self.files.push(path);
            }
        }

        Ok(self)
    }

    pub fn filter_image_files(mut self) -> Self {
        self.files.retain(|file| utils::is_image_file(file));
        self
    }

    pub fn get_image_files_or_none(self) -> Option<Vec<PathBuf>> {
        if self.files.is_empty() {
            None
        } else {
            Some(self.files)
        }
    }
}
