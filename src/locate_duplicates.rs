use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::HashToFiles;
use crate::utils;
use crate::image_files::ImageFiles;


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
