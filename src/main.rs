mod delete_duplicates;
mod files;
mod locate_duplicates;
mod print_duplicates;

use delete_duplicates::delete_duplicate_files;
use files::SHA256FileMap;
use locate_duplicates::compute_sha256_hashes;
use print_duplicates::print_duplicate_files;

use clap::Parser;
use std::path::PathBuf;
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
    loc_duplicates: PathBuf,

    #[arg(short, long, help = "Delete the files (disabled by default)")]
    delete: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut files: SHA256FileMap = match compute_sha256_hashes(cli.loc_duplicates) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    files.isolate_duplicates();

    if files.empty() {
        println!("No duplicates found");
        process::exit(0);
    }

    if cli.delete {
        delete_duplicate_files(files);
    } else {
        print_duplicate_files(files);
    }
}
