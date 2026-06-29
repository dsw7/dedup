mod locate_duplicates;
mod print_duplicates;
mod process_duplicates;
mod types;

use locate_duplicates::compute_sha256_hashes;
use print_duplicates::print_duplicate_files;
use process_duplicates::delete_duplicate_files;
use types::TypeHashes;

use clap::Parser;
use std::path;
use std::process;

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
    loc_duplicates: path::PathBuf,

    #[arg(short, long, help = "Delete the files (disabled by default)")]
    delete: bool,
}

fn main() {
    let cli = Cli::parse();

    let hashes: TypeHashes = match compute_sha256_hashes(cli.loc_duplicates) {
        Ok(hashes) => hashes,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    if hashes.len() < 1 {
        println!("No duplicates found");
        process::exit(0);
    }

    if cli.delete {
        delete_duplicate_files(hashes);
    } else {
        print_duplicate_files(hashes);
    }
}
