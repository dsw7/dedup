use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use anyhow::Context;
use sha2::{Digest, Sha256};

const CHUNK_BUF_SIZE: usize = 65536;

fn compute_file_sha256(file: &PathBuf) -> anyhow::Result<String> {
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

#[derive(Debug)]
pub struct Duplicates {
    hashes: HashMap<String, Vec<PathBuf>>,
}

impl Duplicates {
    pub fn new() -> Self {
        Duplicates {
            hashes: HashMap::new(),
        }
    }

    pub fn get_image_file_hashes(mut self, files: Vec<PathBuf>) -> Self {
        for file in files {
            match compute_file_sha256(&file) {
                Ok(hash) => self.hashes.entry(hash).or_default().push(file),
                Err(error) => {
                    eprintln!("Could not get hash for file '{}': {error}", file.display())
                }
            }
        }
        self
    }

    pub fn isolate_duplicate_files(mut self) -> Self {
        self.hashes.retain(|_, files| files.len() > 1);
        self
    }

    pub fn get_duplicate_files_or_none(self) -> Option<HashMap<String, Vec<PathBuf>>> {
        if self.hashes.is_empty() {
            None
        } else {
            Some(self.hashes)
        }
    }
}
