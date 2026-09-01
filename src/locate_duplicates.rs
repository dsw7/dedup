use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;

use anyhow::Context;

use crate::types::HashToFiles;
use crate::utils;

#[derive(Debug)]
struct ImageFiles {
    files: Vec<PathBuf>,
}

impl ImageFiles {
    fn new() -> Self {
        Self { files: Vec::new() }
    }

    fn locate_all_files(mut self, dir: &PathBuf) -> anyhow::Result<Self> {
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

    fn filter_image_files(mut self) -> Self {
        self.files.retain(|file| utils::is_image_file(file));
        self
    }

    fn get_image_files_or_none(self) -> Option<Vec<PathBuf>> {
        if self.files.is_empty() {
            None
        } else {
            Some(self.files)
        }
    }
}

#[derive(Debug)]
struct Duplicates {
    hashes: HashMap<String, Vec<PathBuf>>,
}

impl Duplicates {
    fn new() -> Self {
        Duplicates {
            hashes: HashMap::new(),
        }
    }

    fn get_image_file_hashes(mut self, files: Vec<PathBuf>) -> Self {
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

    fn locate_duplicate_files(mut self) -> Self {
        self.hashes.retain(|_, files| files.len() > 1);
        self
    }

    fn get_duplicate_files_or_none(self) -> Option<HashMap<String, Vec<PathBuf>>> {
        if self.hashes.is_empty() {
            None
        } else {
            Some(self.hashes)
        }
    }
}

pub fn locate_duplicates(dir: &PathBuf) -> anyhow::Result<Option<HashToFiles>> {
    let image_files = ImageFiles::new()
        .locate_all_files(dir)?
        .filter_image_files()
        .get_image_files_or_none();

    let image_files = match image_files {
        None => {
            println!("No image files in directory");
            return Ok(None);
        }
        Some(files) => files,
    };

    let duplicates = Duplicates::new()
        .get_image_file_hashes(image_files)
        .locate_duplicate_files()
        .get_duplicate_files_or_none();

    Ok(duplicates)
}
