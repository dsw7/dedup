use std::fs::read_dir;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::duplicates::Duplicates;
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

pub fn delete_duplicates(loc_duplicates: PathBuf, dry_run: bool) -> anyhow::Result<&'static str> {
    let all_files = locate_all_files(loc_duplicates)?;
    let image_files = filter_image_files(all_files);

    if image_files.is_empty() {
        return Ok("No image files in directory");
    }

    let duplicates = Duplicates::new()
        .get_image_file_hashes(image_files)
        .isolate_duplicate_files()
        .get_duplicate_files_or_none();

    let duplicates = match duplicates {
        Some(map) => map,
        None => {
            return Ok("No duplicates in directory");
        }
    };

    if dry_run {
        print_duplicate_files(duplicates);
    } else {
        delete_duplicate_files(duplicates);
    }

    Ok("Complete")
}
