use crate::get_file_hash::compute_file_sha256;
use crate::types::TypeHashes;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path;

pub fn compute_sha256_hashes(dir: &path::Path) -> io::Result<TypeHashes> {
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

fn get_index_from_stdin(index: usize) -> io::Result<usize> {
    loop {
        print!("Input an option [0 to {index}]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse() {
            Ok(option) => {
                if option > index {
                    println!("Option cannot exceed {index}. Try again");
                } else {
                    return Ok(option);
                }
                // compiler knows that parse() attempts to convert input into a usize,
                // so -1 will automatically return an Err variant which means no manual
                // check is necessary
            }
            Err(_) => println!("That is not a valid option. Please try again."),
        }
    }
}

fn delete_all_files_except(index_to_keep: usize, files: &Vec<path::PathBuf>) {
    for (index, file) in files.iter().enumerate() {
        if index_to_keep - 1 == index {
            println!(" + {}", file.display());
        } else {
            println!(" - {}", file.display());
        }
    }
}

pub fn delete_duplicate_files(hashes: &TypeHashes) {
    for (hash, files) in hashes {
        println!("Found duplicates with hash: {hash}");
        println!(" [0] -> Skip this batch");

        let mut index = 0;
        for file in files {
            index += 1;
            println!(" [{index}] -> Keep this file: {}", file.display());
        }

        match get_index_from_stdin(index) {
            Ok(option) => {
                if option == 0 {
                    println!("Skipping this batch");
                } else {
                    delete_all_files_except(option, files);
                }
            }
            Err(error) => eprintln!("{error}"),
        }

        println!("");
    }
}
