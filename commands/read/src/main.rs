use std::fs;
use std::env::args;
use std::path::Path;

fn main() {
    let args = args().collect::<Vec<String>>()[1..].join(" ");
    let path = Path::new(&args);

    if args.len() == 0 {
        println!("Usage: read {}", "{file}");
        return;
    }

    match fs::read_to_string(path) {
        Ok(result) => println!("{}", result),
        Err(_) => println!("Unable to read file '{}'", &args)
    }
}