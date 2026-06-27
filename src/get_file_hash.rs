use md5::Context;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

const CHUNK_BUF_SIZE: usize = 4096;

pub fn compute_file_md5(file_path: &Path) -> Result<String> {
    let file = File::open(file_path)?;

    let mut reader = BufReader::new(file);
    let mut context = Context::new();
    let mut buffer = [0u8; CHUNK_BUF_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // eof
        }
        context.consume(&buffer[..bytes_read]);
    }

    let digest = context.compute();
    Ok(format!("{:x}", digest))
}

pub fn compute_file_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
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
