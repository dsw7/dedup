use std::collections::HashMap;
use std::path::PathBuf;

pub type HashToFiles = HashMap<String, Vec<PathBuf>>;

pub fn upsert_hash_and_filepath(hash_to_files: &mut HashToFiles, hash: String, filepath: PathBuf) {
    hash_to_files.entry(hash).or_default().push(filepath);
}

pub fn isolate_duplicates(hash_to_files_all: HashToFiles) -> HashToFiles {
    let hash_to_files_dupes: HashToFiles = hash_to_files_all
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();

    hash_to_files_dupes
}

pub fn empty(hash_to_files: &HashToFiles) -> bool {
    hash_to_files.len() < 1
}
