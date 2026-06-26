use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn traverse_dir(dir: &Path) {
    println!("Path set to: {}", dir.display());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            println!("Directory: {:?}", path);
        } else {
            println!("File: {:?}", path);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let loc_duplicates = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    if !loc_duplicates.exists() {
        eprintln!("The path {:?} does not exist.", loc_duplicates);
        process::exit(1);
    }

    traverse_dir(loc_duplicates);
}
