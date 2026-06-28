mod get_file_hash;
mod locate_duplicates;
mod types;

use locate_duplicates::{compute_sha256_hashes, isolate_duplicate_files, print_duplicate_files};
use types::TypeHashes;

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "dedup", version = "1.0", about = "Deduplicates files in a directory", long_about = None)]
struct Cli {
    #[arg(value_name = "DIR", default_value = ".")]
    loc_duplicates: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut hashes: TypeHashes = match compute_sha256_hashes(&cli.loc_duplicates) {
        Ok(hashes) => hashes,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    isolate_duplicate_files(&mut hashes);

    if hashes.len() < 1 {
        println!("No duplicates found");
        process::exit(0);
    }

    print_duplicate_files(&hashes);
}
