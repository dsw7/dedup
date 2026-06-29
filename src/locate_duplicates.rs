use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path;

use crate::types::TypeHashes;

const CHUNK_BUF_SIZE: usize = 65536;

fn compute_file_sha256<P: AsRef<path::Path>>(path: P) -> io::Result<String> {
    let file = File::open(path)?;

    let mut reader = io::BufReader::new(file);
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

fn append_hash(hashes: &mut TypeHashes, hash: String, file: path::PathBuf) {
    hashes.entry(hash.clone()).or_default().push(file);
}

fn isolate_duplicate_files(hashes: &mut TypeHashes) {
    hashes.retain(|_, files| files.len() > 1);
}

pub fn compute_sha256_hashes(dir: path::PathBuf) -> io::Result<TypeHashes> {
    let mut hashes: TypeHashes = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let filepath = entry.path();
            let hash = compute_file_sha256(&filepath)?;
            append_hash(&mut hashes, hash, filepath);
        }
    }

    isolate_duplicate_files(&mut hashes);
    Ok(hashes)
}
