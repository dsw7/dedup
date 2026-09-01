mod duplicates;
mod image_files;
mod process_duplicates;
mod types;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use duplicates::Duplicates;
use image_files::ImageFiles;
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
    let files_in_dir = ImageFiles::new()
        .locate_all_files(&cli.loc_duplicates)?
        .filter_image_files()
        .get_image_files_or_none();

    let image_files = match files_in_dir {
        Some(files) => files,
        None => {
            println!("No image files in directory");
            return Ok(());
        }
    };

    let duplicates = Duplicates::new()
        .get_image_file_hashes(image_files)
        .isolate_duplicate_files()
        .get_duplicate_files_or_none();

    let duplicates = match duplicates {
        Some(map) => map,
        None => {
            println!("No duplicates in directory");
            return Ok(());
        }
    };

    if cli.dry_run {
        print_duplicate_files(duplicates);
    } else {
        delete_duplicate_files(duplicates);
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
