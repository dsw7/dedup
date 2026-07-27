mod delete_duplicates;
mod errors;
mod get_file_hashes;
mod get_file_sha256;
mod locate_duplicates;
mod print_duplicates;
mod sha256_filemap;
mod types;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use delete_duplicates::delete_duplicate_files;
use get_file_hashes::compute_sha256_hashes;
use locate_duplicates::locate_duplicates;
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

fn main() -> ExitCode {
    let cli = Cli::parse();

    let _ = locate_duplicates(&cli.loc_duplicates);

    let hash_to_files_all: HashToFiles = match compute_sha256_hashes(cli.loc_duplicates) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
    };

    let hash_to_files_dupes: HashToFiles = isolate_duplicates(hash_to_files_all);

    if empty(&hash_to_files_dupes) {
        println!("No duplicates found");
        return ExitCode::SUCCESS;
    }

    if cli.delete {
        delete_duplicate_files(&hash_to_files_dupes);
    } else {
        print_duplicate_files(&hash_to_files_dupes);
    }

    ExitCode::SUCCESS
}
