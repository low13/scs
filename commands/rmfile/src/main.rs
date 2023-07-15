use std::fs;
use std::env::args;
use std::path::Path;

fn main() {
    let args = args().collect::<Vec<String>>()[1..].join(" ");
    if args.len() == 0 {
        println!("Usage: rmfile {}", "{file}");
        return;
    }

    let path = Path::new(&args);
    if path.exists() && path.is_file() {
        let _ = fs::remove_file(&path);
    } else {
        println!("Unable to find file '{}'", &args);
    }
}