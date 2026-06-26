use std::env;
use std::path::Path;
use std::process;

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

    println!("Path set to: {}", loc_duplicates.display());
}
