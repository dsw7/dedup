mod delete_duplicates;
mod locate_duplicates;
mod print_duplicates;
mod sha256_filemap;

use std::path::PathBuf;
use std::process;

use clap::Parser;

use delete_duplicates::delete_duplicate_files;
use locate_duplicates::compute_sha256_hashes;
use print_duplicates::print_duplicate_files;
use sha256_filemap::{HashToFiles, empty, isolate_duplicates};

#[derive(Parser, Debug)]
#[command(
    name = "dedup",
    version,
    about = "Deduplicates image files in a directory",
    after_help = "See https://github.com/dsw7/dedup for more information
See the fdupes(1) manpages for a more general deduplication command"
)]
struct Cli {
    #[arg(value_name = "DIR", default_value = ".")]
    loc_duplicates: PathBuf,

    #[arg(short, long, help = "Delete the files (disabled by default)")]
    delete: bool,
}

fn main() {
    let cli = Cli::parse();

    let hash_to_files_all: HashToFiles = match compute_sha256_hashes(cli.loc_duplicates) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let hash_to_files_dupes: HashToFiles = isolate_duplicates(hash_to_files_all);

    if empty(&hash_to_files_dupes) {
        println!("No duplicates found");
        process::exit(0);
    }

    if cli.delete {
        delete_duplicate_files(&hash_to_files_dupes);
    } else {
        print_duplicate_files(&hash_to_files_dupes);
    }
}
