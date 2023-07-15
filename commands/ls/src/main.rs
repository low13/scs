use std::{env, fs};

fn main() {
    if let Ok(entries) = fs::read_dir(env::current_dir().unwrap()) {
        for entry in entries {
            if let Ok(entry) = entry {
                print!("   {}", entry.file_name().to_string_lossy());
            }
        }
        println!();
    } else {
        println!("Unable to read directory");
    }
}