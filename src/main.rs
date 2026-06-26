use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let loc_duplicates = if args.len() > 1 { &args[1] } else { "." };

    println!("Path set to: {}", loc_duplicates);
}
