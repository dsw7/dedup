use crate::types::TypeHashes;

pub fn print_duplicate_files(hashes: TypeHashes) {
    for (hash, files) in hashes {
        println!("Found duplicates with hash: {hash}");

        for file in files {
            println!("  -> {}", file.display());
        }

        println!("");
    }
}
