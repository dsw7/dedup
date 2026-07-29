use sha2::{Digest, Sha256};

use crate::errors::DedupError;
use crate::types::HashToFiles;

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

const VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];
const CHUNK_BUF_SIZE: usize = 65536;

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

fn is_image_file(filepath: &Path) -> bool {
    filepath
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| VALID_EXTENSIONS.contains(&ext))
}

fn isolate_image_files(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|file| is_image_file(file))
        .collect()
}

fn compute_file_sha256(file: &PathBuf) -> io::Result<String> {
    let file_handle = fs::File::open(file)?;

    let mut reader = BufReader::new(file_handle);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_BUF_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    Ok(format!("{hash_result:x}"))
}

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

pub fn locate_duplicates(dir: &PathBuf) -> Result<HashToFiles, DedupError> {
    let files = locate_all_files(dir)?;
    let image_files = isolate_image_files(files);

    if image_files.is_empty() {
        return Err(DedupError::FilesNotFound(String::from(
            "No image files in directory",
        )));
    }

    let hashes = get_image_file_sha256_hashes(image_files);
    Ok(isolate_duplicate_sha256_hashes(hashes))
}
