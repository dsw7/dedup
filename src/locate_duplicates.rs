use crate::get_file_hash::compute_file_sha256;
use crate::types::TypeSHA256Hashes;

use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn compute_sha256_hashes(dir: &Path) -> Result<TypeSHA256Hashes> {
    let mut hashes: TypeSHA256Hashes = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let filepath = entry.path();

            match compute_file_sha256(&filepath) {
                Ok(hash) => {
                    hashes.entry(hash.clone()).or_default().push(filepath);
                }
                Err(error) => return Err(error),
            };
        }
    }

    Ok(hashes)
}

pub fn isolate_duplicate_files(hashes: &mut TypeSHA256Hashes) {
    hashes.retain(|_, files| files.len() > 1);
}

pub fn delete_duplicate_files(hashes: &TypeSHA256Hashes) {
    for (hash, files) in hashes {
        println!("{hash}:");
        for file in files {
            println!("-> {}", file.display());
        }
    }
}
