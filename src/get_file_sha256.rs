use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

const CHUNK_BUF_SIZE: usize = 65536;

fn compute_file_sha256(file: &PathBuf) -> io::Result<String> {
    let file_handle = File::open(file)?;

    let mut reader = io::BufReader::new(file_handle);
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
    Ok(format!("{:x}", hash_result))
}
