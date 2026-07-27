use crate::errors::DeduplicationError;

use std::fs;
use std::io;
use std::path::PathBuf;

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
    static VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

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

pub fn locate_duplicates(dir: &PathBuf) -> Result<(), DeduplicationError> {
    let files = match locate_all_files(dir) {
        Ok(files) => files,
        Err(e) => return Err(DeduplicationError(format!("{e}"))),
    };

    let image_files = isolate_image_files(files);

    for file in image_files {
        println!("{}", file.display());
    }

    Ok(())
}
