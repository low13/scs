use std::fs;
use std::path::Path;
use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>().join(" ");
    let path = Path::new(&args);
    if path.exists() {
        println!("'{}' already exists", &args);
    } else {
        let _ = fs::create_dir_all(&args);
    }
}