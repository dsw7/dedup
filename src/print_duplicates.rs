use crate::sha256_filemap::HashToFiles;

pub fn print_duplicate_files(hash_to_files_dupes: &HashToFiles) {
    for (hash, filenames) in hash_to_files_dupes {
        println!("Found duplicates with hash: {hash}");

        for file in filenames {
            println!("  -> {}", file.display());
        }

        println!("");
    }
}
