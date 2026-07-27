use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::types::HashToFiles;

fn print_option_range(max_index: usize) {
    print!("Input an option [0 to {max_index}]: ");

    io::stdout()
        .flush()
        .expect("Unrecoverable error: Failed to flush stdout");
}

fn read_input_from_stdin() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unrecoverable error: Failed to read stdin");

    input
}

fn get_option_from_stdin(max_index: usize) -> usize {
    loop {
        print_option_range(max_index);
        let input = read_input_from_stdin();

        match input.trim().parse() {
            Ok(option) => {
                if option > max_index {
                    println!("Option cannot exceed {index}. Try again");
                } else {
                    return option;
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
    match fs::remove_file(file) {
        Ok(_) => println!(" (-) {}", file.display()),
        Err(error) => eprintln!("Cannot delete file '{}': {error}", file.display()),
    }
}

fn delete_all_files_except(index_to_keep: usize, files: &Vec<PathBuf>) {
    for (index, file) in files.iter().enumerate() {
        if index_to_keep - 1 == index {
            println!(" (+) {}", file.display());
        } else {
            delete_single_file(file);
        }
    }
}

fn process_batch_of_duplicates(duplicate_files: &Vec<PathBuf>) {
    let mut index = 0;
    println!(" [{index}] -> Skip this batch");

    for file in duplicate_files {
        index += 1;
        println!(" [{index}] -> Keep this file: {}", file.display());
    }

    let option = get_option_from_stdin(index);

    if option == 0 {
        println!("Skipping this batch");
    } else {
        delete_all_files_except(option, duplicate_files);
    }
}

pub fn delete_duplicate_files(duplicates: &HashToFiles) {
    for (hash, filenames) in duplicates {
        println!("Found duplicates with hash: {hash}");
        process_batch_of_duplicates(&filenames);
        println!("");
    }
}
