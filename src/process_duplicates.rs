use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn get_option_from_stdin(max_index: usize) -> usize {
    loop {
        print!("Input an option [0 to {max_index}]: ");
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Something went wrong when working with I/O");
            continue;
        }

        let Ok(option) = input.trim().parse::<usize>() else {
            println!("Not a valid option. Try again");
            continue;
        };

        if option <= max_index {
            return option;
        }
        println!("Option cannot exceed {max_index}. Try again");
    }
}

fn delete_single_file(file: &PathBuf) {
    match fs::remove_file(file) {
        Ok(_) => println!(" (-) {}", file.display()),
        Err(error) => eprintln!("Cannot delete file '{}': {error}", file.display()),
    }
}

fn delete_all_files_except(index_to_keep: usize, files: &[PathBuf]) {
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

pub fn delete_duplicate_files(duplicates: HashMap<String, Vec<PathBuf>>) {
    for (hash, filenames) in duplicates {
        println!("Found duplicates with hash: {hash}");
        process_batch_of_duplicates(&filenames);
        println!();
    }
}

pub fn print_duplicate_files(duplicates: HashMap<String, Vec<PathBuf>>) {
    for (hash, filenames) in duplicates {
        println!("Found duplicates with hash: {hash}");

        for file in filenames {
            println!("  -> {}", file.display());
        }
        println!();
    }
}
