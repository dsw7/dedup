use std::collections::HashMap;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::get_file_hash::compute_file_sha256;
use crate::process_duplicates::{delete_duplicate_files, print_duplicate_files};

const VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

fn locate_all_files(dir: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let entries = read_dir(&dir).context(format!("failed to read directory: {}", dir.display()))?;
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

fn is_image_file(filepath: &Path) -> bool {
    filepath
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .is_some_and(|ext| VALID_EXTENSIONS.contains(&ext.as_str()))
}

fn filter_image_files(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|file| is_image_file(file))
        .collect()
}

type Hashes = HashMap<String, Vec<PathBuf>>;

fn get_image_file_hashes(files: Vec<PathBuf>) -> Hashes {
    let mut hashes: Hashes = HashMap::new();

    for file in files {
        match compute_file_sha256(&file) {
            Ok(hash) => hashes.entry(hash).or_default().push(file),
            Err(error) => {
                eprintln!("Could not get hash for file '{}': {error}", file.display())
            }
        }
    }
    hashes
}

fn isolate_duplicate_hashes(hashes: Hashes) -> Hashes {
    hashes
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect()
}

pub fn delete_duplicates(loc_duplicates: PathBuf, dry_run: bool) -> anyhow::Result<&'static str> {
    let all_files = locate_all_files(loc_duplicates)?;
    let image_files = filter_image_files(all_files);

    if image_files.is_empty() {
        return Ok("No image files in directory");
    }

    let all_hashes = get_image_file_hashes(image_files);
    let duplicate_hashes = isolate_duplicate_hashes(all_hashes);

    if duplicate_hashes.is_empty() {
        return Ok("No duplicates in directory");
    }

    if dry_run {
        print_duplicate_files(duplicate_hashes);
    } else {
        delete_duplicate_files(duplicate_hashes);
    }

    Ok("Complete")
}
