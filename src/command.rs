use std::{process, env};
use std::path::Path;

pub fn exit() {
    process::exit(0);
}

pub fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "))
}

pub fn cd(args: Vec<&str>) -> Result<(), String> {
    let path = Path::new(args[0]);
    match env::set_current_dir(path) {
        Ok(()) => Ok(()),
        Err(_) => Err("Unable to access directory".to_string())
    }
}