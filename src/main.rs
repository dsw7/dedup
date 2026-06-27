mod get_file_hash;
mod locate_duplicates;
mod types;

use locate_duplicates::{
    compute_md5_hashes, compute_sha256_hashes, delete_duplicate_files, isolate_duplicate_files,
};
use types::TypeHashes;

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

    let use_md5 = true;

    let mut hashes: TypeHashes = if use_md5 {
        match compute_md5_hashes(loc_duplicates) {
            Ok(hashes) => hashes,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
    } else {
        match compute_sha256_hashes(loc_duplicates) {
            Ok(hashes) => hashes,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
    };

    isolate_duplicate_files(&mut hashes);

    if hashes.len() < 1 {
        println!("No duplicates found");
        process::exit(0);
    }

    delete_duplicate_files(&hashes);
}
