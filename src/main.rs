mod get_file_hash;
mod inspect_directory;
mod types;

use inspect_directory::{delete_duplicate_files, locate_duplicate_files};
use types::TypeSHA256Hashes;

use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let loc_duplicates = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    let hashes: TypeSHA256Hashes = match locate_duplicate_files(loc_duplicates) {
        Ok(hashes) => hashes,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    delete_duplicate_files(&hashes);
}
