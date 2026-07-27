use crate::types::HashToFiles;

pub fn print_duplicate_files(duplicates: &HashToFiles) {
    for (hash, filenames) in duplicates {
        println!("Found duplicates with hash: {hash}");

        for file in filenames {
            println!("  -> {}", file.display());
        }

        println!("");
    }
}
