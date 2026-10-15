mod delete_duplicates;
mod duplicates;
mod process_duplicates;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use delete_duplicates::delete_duplicates;

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

fn main() -> ExitCode {
    let cli = Cli::parse();

    match delete_duplicates(cli.loc_duplicates, cli.dry_run) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
