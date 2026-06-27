use crate::get_file_hash::compute_file_sha256;
use crate::types::TypeSHA256Hashes;

use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn locate_duplicate_files(dir: &Path) -> Result<TypeSHA256Hashes> {
    let mut hashes: TypeSHA256Hashes = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let filepath = entry.path();
            match compute_file_sha256(&filepath) {
                Ok(hash) => {
                    hashes.insert(hash, filepath);
                }
                Err(error) => return Err(error),
            };
        }
    }

    Ok(hashes)
}

pub fn delete_duplicate_files(hashes: &TypeSHA256Hashes) {
    for (key, value) in hashes {
        println!("{}: {}", key, value.display());
    }
}
