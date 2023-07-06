use std::{process, env, fs};
use std::path::Path;

pub fn exit() {
    process::exit(0);
}

pub fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "))
}

pub fn cd(args: Vec<&str>) {
    let path = Path::new(args[0]);
    if let Err(_) = env::set_current_dir(path) {
        println!("Unable to access directory");
    }
}

pub fn ls() {
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