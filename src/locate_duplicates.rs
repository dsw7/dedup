use crate::sha256_filemap::SHA256FileMap;

use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Result};
use std::path::{Path, PathBuf};

const CHUNK_BUF_SIZE: usize = 65536;

fn compute_file_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
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

fn is_valid_file_type(file: &PathBuf) -> bool {
    static VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

    match file.extension().and_then(OsStr::to_str) {
        Some(ext) => VALID_EXTENSIONS.contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}

pub fn compute_sha256_hashes(dir: PathBuf) -> Result<SHA256FileMap> {
    let mut files = SHA256FileMap::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if !metadata.is_file() {
            continue;
        }

        let filepath = entry.path();

        if !is_valid_file_type(&filepath) {
            continue;
        }

        let filehash = compute_file_sha256(&filepath)?;
        files.upsert_hash(filehash, filepath);
    }

    Ok(files)
}
