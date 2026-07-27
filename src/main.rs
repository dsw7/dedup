mod delete_duplicates;
mod errors;
mod locate_duplicates;
mod print_duplicates;
mod types;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use delete_duplicates::delete_duplicate_files;
use errors::DedupError;
use locate_duplicates::locate_duplicates;
use print_duplicates::print_duplicate_files;

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

fn process_directory() -> Result<(), DedupError> {
    let cli = Cli::parse();
    let hashes = locate_duplicates(&cli.loc_duplicates)?;

    if cli.delete {
        delete_duplicate_files(&hashes);
    } else {
        print_duplicate_files(&hashes);
    }

    Ok(())
}

fn main() -> ExitCode {
    match process_directory() {
        Ok(()) => {
            println!("Complete!");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
