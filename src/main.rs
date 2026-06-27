mod get_file_hash;
mod locate_duplicates;
mod types;

use locate_duplicates::{compute_all_file_hashes, isolate_duplicate_files};
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

    let hashes: TypeSHA256Hashes = match compute_all_file_hashes(loc_duplicates) {
        Ok(hashes) => hashes,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    isolate_duplicate_files(&hashes);
}
