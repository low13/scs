use std::fs;
use std::env::args;
use std::path::Path;

fn main() {
    let args = args().collect::<Vec<String>>()[1..].join(" ");

    if args.len() == 0 {
        println!("Usage: mkfile {}", "{file}");
        return;
    }

    let path = Path::new(&args);
    if path.exists() {
        println!("'{}' already exists", &args);
    } else {
        let _ = fs::File::create(&args);
    }
}