use crate::errors::DeduplicationError;
use crate::get_file_sha256::compute_file_sha256;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

const VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

fn locate_all_files(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn is_image_file(filepath: &PathBuf) -> bool {
    filepath
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| VALID_EXTENSIONS.contains(&ext))
}

fn isolate_image_files(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|file| is_image_file(&file))
        .collect()
}

type HashToFiles = HashMap<String, Vec<PathBuf>>;

fn get_image_file_sha256_hashes(files: Vec<PathBuf>) -> HashToFiles {
    let mut hashes: HashToFiles = HashMap::new();

    for file in files {
        match compute_file_sha256(&file) {
            Ok(hash) => hashes.entry(hash).or_default().push(file),
            Err(e) => eprintln!("Could not get hash for file '{}': {e}", file.display()),
        }
    }

    hashes
}

fn isolate_duplicate_sha256_hashes(hashes: HashToFiles) -> HashToFiles {
    hashes
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect()
}

pub fn locate_duplicates(dir: &PathBuf) -> Result<String, DeduplicationError> {
    let files = match locate_all_files(dir) {
        Ok(files) => files,
        Err(e) => return Err(DeduplicationError(format!("{e}"))),
    };

    let image_files = isolate_image_files(files);

    if image_files.is_empty() {
        return Ok(String::from("No image files in directory"));
    }

    let hashes = get_image_file_sha256_hashes(image_files);
    let _ = isolate_duplicate_sha256_hashes(hashes);

    Ok(String::from("Complete!"))
}
