use crate::get_file_hash::compute_file_sha256;
use crate::types::TypeHashes;

use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn compute_sha256_hashes(dir: &Path) -> Result<TypeHashes> {
    let mut hashes: TypeHashes = HashMap::new();

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

pub fn isolate_duplicate_files(hashes: &mut TypeHashes) {
    hashes.retain(|_, files| files.len() > 1);
}

pub fn print_duplicate_files(hashes: &TypeHashes) {
    for (hash, files) in hashes {
        println!("Found duplicates with hash: {hash}");
        for file in files {
            println!("  -> {}", file.display());
        }
        println!("");
    }
}
