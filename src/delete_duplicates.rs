use crate::sha256_filemap::SHA256FileMap;

use std::fs::remove_file;
use std::io::{self, Result, Write};
use std::path::PathBuf;

fn get_index_from_stdin(index: usize) -> Result<usize> {
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

fn delete_single_file(file: &PathBuf) {
    match remove_file(file) {
        Ok(_) => println!(" (-) {}", file.display()),
        Err(error) => eprintln!("Cannot delete file '{}': {error}", file.display()),
    }
}

fn delete_all_files_except(index_to_keep: usize, files: Vec<PathBuf>) {
    for (index, file) in files.iter().enumerate() {
        if index_to_keep - 1 == index {
            println!(" (+) {}", file.display());
        } else {
            delete_single_file(file);
        }
    }
}

pub fn delete_duplicate_files(files: SHA256FileMap) {
    for (hash, filenames) in files.hashes {
        println!("Found duplicates with hash: {hash}");
        println!(" [0] -> Skip this batch");

        let mut index = 0;
        for file in &filenames {
            index += 1;
            println!(" [{index}] -> Keep this file: {}", file.display());
        }

        match get_index_from_stdin(index) {
            Ok(option) => {
                if option == 0 {
                    println!("Skipping this batch");
                } else {
                    delete_all_files_except(option, filenames);
                }
            }
            Err(error) => eprintln!("{error}"),
        }

        println!("");
    }
}
