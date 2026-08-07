mod locate_duplicates;
mod process_duplicates;
mod types;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use locate_duplicates::locate_duplicates;
use process_duplicates::{delete_duplicate_files, print_duplicate_files};

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

fn process_directory() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let duplicates = match locate_duplicates(&cli.loc_duplicates)? {
        Some(hashes) => hashes,
        None => return Ok(()),
    };

    if cli.delete {
        delete_duplicate_files(duplicates);
    } else {
        print_duplicate_files(duplicates);
    }

    Ok(())
}

fn main() -> ExitCode {
    match process_directory() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
