use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use anyhow::Context;
use sha2::{Digest, Sha256};

const VALID_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];
const CHUNK_BUF_SIZE: usize = 65536;

pub fn is_image_file(filepath: &Path) -> bool {
    filepath
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .is_some_and(|ext| VALID_EXTENSIONS.contains(&ext.as_str()))
}

pub fn compute_file_sha256(file: &PathBuf) -> anyhow::Result<String> {
    let file_handle =
        File::open(file).context(format!("failed to open file: {}", file.display()))?;

    let mut reader = BufReader::new(file_handle);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_BUF_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).context(format!(
            "failed to read bytes from file: {}",
            file.display()
        ))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    Ok(format!("{hash_result:x}"))
}
