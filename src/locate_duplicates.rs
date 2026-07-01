use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use crate::sha256_filemap::{HashToFiles, upsert_hash_and_filepath};

const CHUNK_BUF_SIZE: usize = 65536;

fn is_valid_file_type(file: &PathBuf) -> bool {
    static VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

    match file.extension().and_then(OsStr::to_str) {
        Some(ext) => VALID_EXTENSIONS.contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}

fn compute_file_sha256(file: &PathBuf) -> Result<String> {
    let file_handle = File::open(file)?;

    let mut reader = BufReader::new(file_handle);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_BUF_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // eof
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

pub fn compute_sha256_hashes(dir: PathBuf) -> Result<HashToFiles> {
    let mut hash_to_files_all: HashToFiles = HashMap::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if !metadata.is_file() {
            continue;
        }

        let file = entry.path();

        if !is_valid_file_type(&file) {
            continue;
        }

        let hash = compute_file_sha256(&file)?;
        upsert_hash_and_filepath(&mut hash_to_files_all, hash, file);
    }

    Ok(hash_to_files_all)
}
