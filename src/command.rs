use std::{process, env, fs};
use std::path::Path;

pub fn exit() {
    process::exit(0);
}

pub fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}

pub fn cd(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
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

pub fn mkfile(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
    if path.exists() {
        println!("'{}' already exists", &args);
    } else {
        let _ = fs::File::create(&args);
    }
}

pub fn rmfile(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
    if path.exists() && path.is_file() {
        let _ = fs::remove_file(&path);
    } else {
        println!("Unable to find file '{}'", &args);
    }
}

pub fn mkdir(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
    if path.exists() {
        println!("'{}' already exists", &args);
    } else {
        let _ = fs::create_dir_all(&args);
    }
}

pub fn rmdir(args: Vec<&str>) {
    let args_temp;
    let mut force = false;

    if args[0] == "--force" {
        force = true;
        args_temp = args[1..].join(" ");
    } else {
        args_temp = args.join(" ");
    }

    let args = args_temp;

    let path = Path::new(&args);
    if path.exists() && path.is_dir() {
        if let Ok(entries) = fs::read_dir(&path) {
            if entries.count() == 0 {
                let _ = fs::remove_dir(&path);
            } else if force {
                let _ = fs::remove_dir_all(&path);
            } else {
                println!("Directory '{}' is not empty. Consider using '--force' to delete it anyway", args);
            }
        }
    } else {
        println!("Unable to find directory '{}'", &args);
    }
}