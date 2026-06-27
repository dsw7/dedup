mod get_file_hash;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::process;

fn locate_duplicate_files(dir: &Path) -> Result<()> {
    let mut hashes = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let filepath = entry.path();
            match get_file_hash::compute_file_sha256(&filepath) {
                Ok(hash) => hashes.insert(hash, filepath),
                Err(_) => todo!(),
            };
        }
    }

    for (key, value) in &hashes {
        println!("{}: {}", key, value.display());
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let loc_duplicates = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    match locate_duplicate_files(loc_duplicates) {
        Ok(()) => println!("Success!"),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
}
