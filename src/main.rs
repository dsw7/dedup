use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::process;

fn traverse_dir_with_duplicates(dir: &Path) -> Result<String> {
    let mut count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let path = entry.path();
            println!("File: {:?}", path);
            count += 1;
        }
    }

    Ok(format!("\nScanned {count} files"))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let loc_duplicates = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    match traverse_dir_with_duplicates(loc_duplicates) {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
}
