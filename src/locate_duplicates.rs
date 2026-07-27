use crate::errors::DeduplicationError;

use std::fs;
use std::io;
use std::path::PathBuf;

fn locate_files(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

pub fn locate_duplicates(dir: &PathBuf) -> Result<(), DeduplicationError> {
    let files = match locate_files(dir) {
        Ok(files) => files,
        Err(e) => return Err(DeduplicationError(format!("{e}"))),
    };

    for file in files {
        println!("{}", file.display());
    }

    Ok(())
}
