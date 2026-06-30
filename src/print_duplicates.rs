use crate::sha256_filemap::SHA256FileMap;

pub fn print_duplicate_files(files: SHA256FileMap) {
    for (hash, filenames) in files.hashes {
        println!("Found duplicates with hash: {hash}");

        for file in filenames {
            println!("  -> {}", file.display());
        }

        println!("");
    }
}
