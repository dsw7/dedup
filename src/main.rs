mod get_duplicates;
mod process_duplicates;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use get_duplicates::get_duplicates;
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

    #[arg(long, help = "Dry run (print duplicates without deleting)")]
    dry_run: bool,
}

fn process_directory(cli: Cli) -> anyhow::Result<()> {
    if let Some(hashes) = get_duplicates(cli.loc_duplicates)? {
        if cli.dry_run {
            print_duplicate_files(hashes);
        } else {
            delete_duplicate_files(hashes);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match process_directory(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
